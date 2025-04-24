use clap::{Args, Parser, command};
use dfwasm_compiler::{DFWasmCompiler, DFWasmCompilerOptions};
use dfwasm_template::{Template, template_sender::DFApiClient};

#[derive(Parser, Debug)]
#[command(
    name = "DFWasm CLI",
    version = "0.1.0",
    about = "A compiler from WebAssembly to DiamondFire templates."
)]
struct CLIArgs {
    #[arg(short, long, help = "Include debugger function calls")]
    debugger: bool,
    #[arg(short, long, default_value = "301", help = "The DiamondFire plot size")]
    size: usize,
    #[arg(
        short,
        long,
        num_args(0..=1),
        default_missing_value = "26",
        help = "Batch data section memory initializations by a certain size"
    )]
    batch_data: Option<usize>,

    #[command(flatten)]
    output: OutputMethod,

    #[arg(help = "The path to the WebAssembly file")]
    path: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct OutputMethod {
    #[arg(
        short,
        long,
        help = "Send templates via CodeClient API",
        group = "output"
    )]
    cc: bool,
    #[arg(
        short,
        long,
        help = "Send templates via dfonline.dev links",
        group = "output"
    )]
    link: bool,
}

#[tokio::main]
async fn main() {
    let cli_args = CLIArgs::parse();

    let bytes = std::fs::read(&cli_args.path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", cli_args.path));

    let templates = DFWasmCompiler::wasm_to_template(
        &bytes,
        DFWasmCompilerOptions {
            module_name: None,
            debugger: cli_args.debugger,
            skip_nop_debugger: false,
            max_template_size: Some(cli_args.size),
            batch_data: cli_args.batch_data.is_some(),
            batch_data_size: cli_args.batch_data,
            only_include_module_init: false,
        },
    )
    .expect("Failed to parse wasm file");

    if cli_args.output.cc {
        send_templates_to_cc(&templates).await;
    } else if cli_args.output.link {
        for template in &templates {
            let link = format!(
                "https://dfonline.dev/edit?template={}",
                template.encode().unwrap()
            );

            println!("{link}");
        }
    }
}

async fn send_templates_to_cc(templates: &[Template]) {
    let mut client = DFApiClient::connect().await.unwrap();
    client.send_templates(templates, false).await.unwrap();
}
