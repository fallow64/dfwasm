mod test_builder;
mod util;

use std::{env, path::PathBuf};

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
            .into_iter()
            .map(PathBuf::from)
            .filter(|path| {
                path.extension()
                    .map_or(false, |ext| ext == "wat" || ext == "wasm")
            })
            .collect()
    };

    for test_file in wat_files {
        if !test_file.exists() {
            eprintln!("File not found: {}", test_file.display());
            continue;
        }

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
    let mut root_template = Template::start_function("root_test".to_string());

    // Create a list of the test functions to call
    let mut function_list_buffer = Vec::new();
    for test in &module_tests {
        function_list_buffer.push(Item::string(test.main_function_name.clone()));

        if function_list_buffer.len() == 26 {
            let mut append_args = vec![Item::var("test_functions")];
            append_args.extend(function_list_buffer.clone());

            root_template.set_var("AppendValue", Args::with(append_args));
            function_list_buffer.clear();
        }
    }

    // Ensure the buffer is not empty
    if !function_list_buffer.is_empty() {
        let mut append_args = vec![Item::var("test_functions")];
        append_args.extend(function_list_buffer);
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

    // Split the templates.
    // The compiler already splits its functions, however the root template and the templates
    // containg the test cases are not split.
    let templates = split_templates(templates, 301);

    templates
}

async fn send_templates_to_cc(templates: &[Template]) -> Result<()> {
    let mut client = DFApiClient::connect().await?;
    client.send_templates(templates, false).await?;

    Ok(())
}
