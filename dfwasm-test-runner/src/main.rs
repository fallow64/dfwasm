mod test_builder;
mod util;

use std::{collections::HashSet, env, path::PathBuf};

use anyhow::Result;
use dfwasm_compiler::DFWasmCompilerOptions;
use dfwasm_template::{
    Args, CodeBlock, Item, Template, split_templates, template_sender::DFApiClient,
};
use test_builder::compile_module_from_path;
use util::CompiledModuleTest;

pub const PLOT_SIZE: usize = 301;
// todo: CLI Args for this?
pub const TEST_COMPILER_OPTIONS: DFWasmCompilerOptions = DFWasmCompilerOptions {
    module_name: None,
    debugger: false,
    skip_nop_debugger: false,
    max_template_size: Some(PLOT_SIZE),
    batch_data_size: None,
    only_include_module_init: false,
    include_wait: true,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut compiled_module_tests = Vec::new();

    let cli_args: Vec<_> = env::args().collect();

    let wat_files: Vec<PathBuf> = if cli_args.len() == 1 {
        // Glob under test_files directory
        glob::glob("test_files/**/*.wat")
            .expect("Valid glob pattern")
            .collect::<Result<Vec<_>, _>>()?
    } else {
        // Use the provided arguments as file paths
        cli_args[1..]
            .iter()
            .map(PathBuf::from)
            .filter(|path| {
                path.extension()
                    .is_some_and(|ext| ext == "wat" || ext == "wasm")
            })
            .collect()
    };

    let mut modules_compiled = HashSet::new();
    for test_file in wat_files {
        if !test_file.exists() {
            eprintln!("File not found: {}", test_file.display());
            continue;
        }

        let file_stem = test_file
            .file_stem()
            .expect("Failed to get file stem")
            .to_str()
            .expect("Failed to convert file stem to string");

        if modules_compiled.contains(file_stem) {
            // Skip if a module by the same name has already been compiled
            // i.e. `xyz.wat` may be a decompilation of `xyz.wasm`
            eprintln!("Module already compiled: {file_stem}");
            continue;
        }
        modules_compiled.insert(file_stem.to_string());

        let compiled_test = compile_module_from_path(&test_file).unwrap_or_else(|e| {
            panic!("Failed to compile test file {}: {}", test_file.display(), e)
        });

        compiled_module_tests.push(compiled_test);
    }

    // Create the root template for the module tests
    let templates = create_root_template(compiled_module_tests);

    // Send the templates to CodeClient
    send_templates_to_cc(&templates).await?;

    Ok(())
}

/// Creates the root template for the module tests.
/// Splits the module tests into multiple templates if they exceed the maximum size.
fn create_root_template(module_tests: Vec<CompiledModuleTest>) -> Vec<Template> {
    let mut root_template = Template::start_function("wasm.test".to_string());

    let module_test_function_names = module_tests
        .iter()
        .map(|test| test.main_function_name.clone())
        .collect::<Vec<_>>();

    // Create a list of test function names
    // Chunk using 26 (27 args per block, including the test_functions var) to lessen code size
    for name_chunk in module_test_function_names.chunks(26) {
        let mut append_args = vec![Item::var("test_functions")];
        append_args.extend(name_chunk.iter().map(|name| Item::string(name.clone())));

        root_template.set_var("AppendValue", Args::with(append_args));
    }

    // Now, repeat through that list and call each function
    root_template
        .repeat(
            "ForEach",
            Args::with(vec![
                Item::var("test_function"),
                Item::var("test_functions"),
            ]),
        )
        .open_bracket_repeat()
        .call_function("%var(test_function)", Args::default())
        .control(
            "Wait",
            Args::with_tags(
                vec![Item::num(1)],
                vec![Item::Tag {
                    option: "Ticks".to_string(),
                    tag: "Time Unit".to_string(),
                    action: "Wait".to_string(),
                    block: CodeBlock::Control,
                }],
            ),
        )
        .close_bracket_repeat();

    let mut templates = vec![root_template];
    // Add the module templates code to the root template
    templates.extend(module_tests.into_iter().flat_map(|test| test.templates));

    // Return the result of splitting the templates
    // The compiler already splits its functions, however the root template and the templates
    // containg the test cases are not split.
    split_templates(templates, 301)
}

async fn send_templates_to_cc(templates: &[Template]) -> Result<()> {
    let mut client = DFApiClient::connect().await?;
    client.send_templates(templates, false).await?;

    Ok(())
}
