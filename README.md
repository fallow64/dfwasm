# dfwasm: A WebAssembly to DiamondFire Compiler

`dfwasm` is a compiler that compiles [WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly) (WASM) to [DiamondFire](https://mcdiamondfire.com) (DF) templates. It is designed to translate WASM modules into several templates, which can be used within DiamondFire code.

This allows you to write code in a language that compiles to WASM, and then use that code within DiamondFire. <br/>
<sup><sub>Yes, that means you can run blazingly fast Rust in DiamondFire.</sub></sup>

> [!NOTE]
> This compiler does not support floating point numbers. Numbers in DiamondFire are represented as fixed-point integers with a 1/1000 scale, so supporting floats would require implementing fixed-point math from scratch. While this would be possible, it would not be performant within DiamondFire.
> 
> While I haven't looked into it, a concept called "soft floats" may be used to get around this.

## How to Use

### Obtaining a WASM file

You will first need a `.wasm` file. There are several ways to compile to WASM. Here are a few options:
- AssemblyScript: A TypeScript-like language that compiles to WASM.
- Rust: A systems programming language (that this project is written in) that can compile to WASM. See [this thread](https://stackoverflow.com/questions/60980310/how-do-i-compile-a-rust-project-to-wasm-without-using-wasm-pack) for how to compile a Rust project without using wasm-pack or wasm-bindgen.
- C/C++: You can use Emscripten or clang to compile C/C++ code to WASM.
- Many more options are available. Here is a [list of languages that can compile to WASM](https://github.com/appcypher/awesome-wasm-langs).

Whichever option you choose, I highly recommend using a language with a small runtime and features to disable the standard library. This will help reduce the size of the generated WASM file and let it fit within a DiamondFire plot.

For some examples that already work, check out the [`examples`](/examples) directory.

### Using `dfwasm`

In the root directory, run `cargo run` to run the compiler CLI.

The arguments are as follows (may be updated, check `dfwasm --help` for help):
- ```
  Usage: dfwasm [OPTIONS] <--cc|--link> <PATH>

  Arguments:
  <PATH>  The path to the WebAssembly (or WAT) file

  Options:
  -d, --debugger                   Include debugger function calls
  -s, --size <SIZE>                The DiamondFire plot size [default: 301]
  -b, --batch-data [<BATCH_DATA>]  Batch data section memory initializations by a certain size
  -c, --cc                         Send templates via CodeClient API
  -l, --link                       Send templates via dfonline.dev links
  -h, --help                       Print help
  -V, --version                    Print version
  ```

The project is divided into several Cargo packages:

1. `dfwasm`: The CLI to interface with the compiler.
2. `dfwasm-compiler`: The core compiler that handles the translation from WASM to DF.
3. `dfwasm-template`: A serde_json serializer/deserializer for DF templates, including an optional `codeclient` feature to send templates via the [CodeClient](https://github.com/DFOnline/CodeClient) API.
4. `dfwasm-test-runner`: A test runner that compiles many modules, runs them, and compares the output to `wasmer` (a WASM runtime).

## Roadmap

- [x] An automated test suite.
- [ ] Add the [utility functions](/dfwasm-compiler/src/df_helper.rs#L=241) to the compiler.
- [ ] Allow multiple modules to be initialized and ran at once.
- [ ] Extract commonly used lists of instructions into a different function to reduce code duplication.
- [ ] Combine multiple small templates into one large template that fits within the size limit.
      This will lessen the burden of CodeClient's template placer.
- [ ] Run doom in DiamondFire (the inspiration for this project).
- [ ] Add support for WASM floats. (stretch goal)

## Acknowledgements

This project is heavily inspired by [MichiganTypeScript](https://www.youtube.com/@MichiganTypeScript)'s [typescript-types-only-wasm-runtime](https://github.com/MichiganTypeScript/typescript-types-only-wasm-runtime/tree/master), which is a marvel of a TypeScript project that runs WASM entirely within the TypeScript type system. Many ideas, concepts, and tests were taken from that project.

## License

This project is licensed under the [MIT License](LICENSE).