use std::{fs, path::Path};

use anyhow::Result;
use dfwasm_compiler::{DFWasmCompiler, DFWasmCompilerOptions};
use dfwasm_template::{Args, Item, Location, Template};
use wasmer::{Instance, Module, Store, wat2wasm};

use crate::{
    TEST_COMPILER_OPTIONS,
    util::{CompiledModuleTest, TestCase, clear_variables, get_wasmer_results, parse_input_file},
};

/// Compiles a module to test from a WAT file.
///
/// Each individual test is separated into two files: `<test_name>.wat` and `<test_name>.test`.
/// The WAT file contains the module's code, while the `.test` file contains test cases.
///
/// Each test case is a function call to some exported function in the module.
/// This is compared to what `wasmer` returns from the same code, and DF code is generated
/// to ensure they have the same result.
pub fn compile_module_from_path(wat_path: &Path) -> Result<CompiledModuleTest> {
    let wat_contents = fs::read(wat_path)?;
    let wasm = wat2wasm(&wat_contents)
        .unwrap_or_else(|e| panic!("Invalid wat file {}: {}", wat_path.display(), e));

    let module_name = wat_path
        .file_stem()
        .expect("file stem")
        .to_str()
        .expect("utf8");

    let cases_path = wat_path.with_extension("test");
    let cases_file = fs::read_to_string(&cases_path).expect("a .test file for test case");

    let cases = parse_input_file(module_name, &cases_file);

    compile_module(module_name, &wasm, &cases)
}

/// Compiles a module to test given the bytes of the WASM file and the test cases.
fn compile_module(test_name: &str, wasm: &[u8], cases: &[TestCase]) -> Result<CompiledModuleTest> {
    let module_function_name = format!("test_{test_name}");
    let mut module_function = Template::start_function(module_function_name.clone());

    // Clear variables
    clear_variables(&mut module_function);

    // Call the module's init (todo: maybe return this from DFWasmCompiler?)
    module_function.call_function(format!("{test_name}_module_init"), Args::default());

    // Send debug message
    module_function.print(Args::with(vec![Item::Text {
        name: format!("<blue>ℹ<white> Running module test: {test_name}"),
    }]));

    // Compile the WASM
    let mut templates = DFWasmCompiler::compile_wasm(
        wasm,
        DFWasmCompilerOptions {
            module_name: Some(test_name.to_string()),
            ..TEST_COMPILER_OPTIONS
        },
    )?;

    // Create a wasmer instance

    // note: since this is shared between all test cases, technically the global state
    // persists between test cases.
    let mut store = Store::default();
    let module = Module::new(&store, wasm)?;
    let import_object = wasmer::imports! {}; // no imports
    let mut instance = Instance::new(&mut store, &module, &import_object)?;

    for case in cases {
        // Compile the individual test case
        compile_test_case(&mut module_function, case, &mut store, &mut instance)?;
    }

    // Add the main test function to the templates
    templates.push(module_function);

    Ok(CompiledModuleTest {
        main_function_name: module_function_name,
        templates,
    })
}

/// Compiles a single test case.
fn compile_test_case(
    module_function: &mut Template,
    case: &TestCase,
    store: &mut Store,
    wasmer_instance: &mut Instance,
) -> Result<()> {
    let (df_inputs, df_expected_results) = get_wasmer_results(case, store, wasmer_instance)?;

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

    module_function.call_function("$expectEqual", Args::with(args));

    Ok(())
}
