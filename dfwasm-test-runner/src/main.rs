mod util;

use std::{
    fs::{self},
    path::Path,
};

use anyhow::Result;
use dfwasm_compiler::{DFWasmCompiler, DFWasmCompilerOptions};
use dfwasm_template::{Args, Item, Location, Template, split_templates};
use util::{
    CompiledModuleTest, TestCase, clear_variables, get_wasmer_info, parse_input_file,
    send_templates_to_cc,
};
use wasmer::{Instance, Module, Store, wat2wasm};

const PLOT_SIZE: usize = 301;

const TEST_COMPILER_OPTIONS: DFWasmCompilerOptions = DFWasmCompilerOptions {
    module_name: None,
    debugger: false,
    skip_nop_debugger: false,
    max_template_size: Some(PLOT_SIZE),
    batch_data: true,
    batch_data_size: None,
    only_include_module_init: false,
};

fn compile_test_case(
    test_function: &mut Template,
    case: &TestCase,
    store: &mut Store,
    wasmer_instance: &mut Instance,
) -> Result<()> {
    let (df_inputs, df_expected_results) = get_wasmer_info(case, store, wasmer_instance)?;

    let mut args = vec![
        Item::string(case.module_name.clone()),
        Item::string(case.function_name.clone()),
        Item::string(case.inputs.join(", ")),
    ];

    args.extend(df_expected_results);
    args.push(Item::Location {
        is_block: false,
        loc: Location::default(),
    });
    args.extend(df_inputs);

    test_function.call_function("$expectEqual", Args::with(args));

    Ok(())
}

fn compile_module_test(
    test_name: &str,
    wasm: &[u8],
    cases: &[TestCase],
) -> Result<CompiledModuleTest> {
    let main_function_name = format!("test_{test_name}");
    let mut main_test_function = Template::start_function(main_function_name.clone());

    // Clear variables
    clear_variables(&mut main_test_function);
    // Call the module init (todo: maybe return this from DFWasmCompiler?)
    main_test_function.call_function(format!("{test_name}_module_init"), Args::default());

    // Send debug message
    msg_running_module_test(&mut main_test_function, test_name);

    // Compile the WASM
    let mut templates = DFWasmCompiler::wasm_to_template(
        wasm,
        DFWasmCompilerOptions {
            module_name: Some(test_name.to_string()),
            ..TEST_COMPILER_OPTIONS
        },
    )?;

    // Create a wasmer instance
    let mut store = Store::default();
    let module = Module::new(&store, wasm)?;
    let import_object = wasmer::imports! {}; // no imports
    let mut instance = Instance::new(&mut store, &module, &import_object)?;

    for case in cases {
        // Compile the individual test case
        compile_test_case(&mut main_test_function, case, &mut store, &mut instance)?;
    }

    // Add the main test function to the templates
    templates.push(main_test_function);

    Ok(CompiledModuleTest {
        main_function_name,
        templates,
    })
}

fn get_wat_module_test(wat_path: &Path) -> Result<CompiledModuleTest> {
    let wat_contents = fs::read(wat_path)?;
    let wasm = wat2wasm(&wat_contents).expect("Valid wat file");

    let module_name = wat_path
        .file_stem()
        .expect("file stem")
        .to_str()
        .expect("utf8");

    let cases_path = wat_path.with_extension("test");
    let cases_file = fs::read_to_string(&cases_path).expect("a .test file for test case");

    let cases = parse_input_file(module_name.to_string(), &cases_file);

    compile_module_test(module_name, &wasm, &cases)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut compiled_module_tests = Vec::new();

    const WAT_GLOB: &str = "**/*.wat";
    for test_file in glob::glob(WAT_GLOB)? {
        let test_file = test_file?;

        let compiled_test = get_wat_module_test(&test_file)?;
        compiled_module_tests.push(compiled_test);
    }

    // Create the "root" test function
    let mut root_test = Template::start_function("root_test".to_string());

    let mut templates = Vec::new();
    for test in compiled_module_tests {
        // Call each function in the root test
        root_test.call_function(test.main_function_name, Args::default());
        // Add the test templates to the root templates
        templates.extend(test.templates);
    }

    // Add the root test to the templates
    templates.push(root_test);

    // Split the templates
    let templates = split_templates(templates, PLOT_SIZE);

    // Send the templates to CodeClient
    send_templates_to_cc(&templates).await?;

    Ok(())
}

//
// -- display methods
//

fn msg_running_module_test(template: &mut Template, test_name: &str) {
    print(
        template,
        format!("<blue>ℹ<white> Running module test: {test_name}"),
    );
}

fn print(template: &mut Template, text: impl Into<String>) {
    template.print(Args::with(vec![Item::Text { name: text.into() }]));
}
