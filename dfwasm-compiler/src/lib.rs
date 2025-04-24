/*
WASM State:
- Compile Time:
    - Functions:
        - Defined functions are indexed
        - Signatures known from Type section
        - Imported functions also occupy indices
    - Table(s)
        - Initial Size and Type (only funcref atm)
        - Maximum Size if specified
        - Static Elements using (elem)
    - Memories
        - Initial and optional max size (in 64KiB pages)
        - Export status
        - Whether shared
        - Contents are not static, but the type of memory is
    - Globals
        - Types, mutability
        - Initial values (whether it be static or import/global)
    - Imports
        - Module and field names
        - Type of import
        - Type signatures of memory/table/global
        - Index of memory/table/global
    - Exports
        - Name (not module name)
        - Type of export (func/memory/global/table)
        - Indices
- At Execution:
    - Imports
        - Resolved into actual references: i.e. functions, memories, tables, or globals
    - Execution Stack / Frame Stack
        - StackFrame[]
        - Locals stack
            - Map<localIndex, Value>
        - Operand stack
            - Value[]
    - Memories
        - Map<memoryIndex, Map<address, Value>>
        - Size of memory (can grow via `memory.grow`)
    - Global(s) State
        - Map<globalIndex, Value>
        - Mutable globals are mutable, immutable variables are constant
    - Table(s) State
        - Map<tableIndex, Value[]>
        - Can be modified at runtime
*/
use thiserror::Error;
use wasmparser::BinaryReaderError;

mod compiler;
mod df_helper;
mod operators;
mod sections;

type DFWasmResult<T> = std::result::Result<T, DFWasmError>;

#[derive(Error, Debug)]
pub enum DFWasmError {
    #[error("error: {0}")]
    ReadError(#[from] BinaryReaderError),

    #[error("error: unsupported WASM version {0}")]
    UnsupportedVersion(u16),
    #[error("error: WASM components are not supported")]
    UnsupportedComponents,
    #[error("error: WASM tags are not supported")]
    UnsupportedTags,
    #[error("error: unsupported const expr")]
    UnsupportedConstExpr,

    #[error("error: unsupported operator {op} at location {location}")]
    UnsupportedOperator { op: String, location: usize },

    #[error("error: tables have a maximum size of 10000")]
    MaximumTableSize,
    #[error("error: multiple memories are not supported")]
    MultipleMemories,
    #[error("error: multiple start methods are not supported")]
    MultipleStartMethods,

    #[error("error: found unknown section with id {0}")]
    UnknownSection(u8),
    #[error("error: unknown payload")]
    UnknownPayload,

    #[error("error: {0} are not yet implemented")]
    NotYetImplemented(&'static str),
}

pub use compiler::{DFWasmCompiler, DFWasmCompilerOptions};
