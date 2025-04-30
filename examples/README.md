# Examples

Here are some example one file programs that can be compiled.

For an entire Rust Cargo project, see [github.com/fallow64/dfwasm-example-project](https://github.com/fallow64/dfwasm-example-project).

## Compiling C to WebAssembly

```bash
clang -Os --target=wasm32 --no-standard-libraries -Wl,--export-all,--no-entry -o life.wasm life.c
```

## Compiling Rust (one file) to WebAssembly

```bash
rustc --target=wasm32-unknown-unknown -C opt-level=s -o bf.wasm bf.rs
```
