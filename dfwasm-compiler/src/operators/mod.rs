use dfwasm_template::Args;
use wasmparser::Operator;

use basic::compile_basic_operator;
use control_flow::compile_control_flow_operator;
use memory::compile_memory_operator;
use numeric::compile_numeric_operator;
use table::compile_table_operator;

use crate::{DFWasmError, DFWasmResult};

use super::{DFWasmCompiler, df_helper::string};

mod basic;
mod control_flow;
mod memory;
mod numeric;
mod table;

/// Compile a single operator into the current template (and potentially add new templates).
pub fn compile_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    location: usize,
) -> DFWasmResult<()> {
    handle_debugger_call(compiler, &operator, location);

    // Call the appropriate function based on the operator type.
    match &operator {
        Operator::Nop
        | Operator::Call { .. }
        | Operator::CallIndirect { .. }
        | Operator::Drop
        | Operator::Select
        | Operator::TypedSelect { .. }
        | Operator::LocalGet { .. }
        | Operator::LocalSet { .. }
        | Operator::LocalTee { .. }
        | Operator::GlobalGet { .. }
        | Operator::GlobalSet { .. } => compile_basic_operator(compiler, operator, location),
        Operator::Block { .. }
        | Operator::Loop { .. }
        | Operator::If { .. }
        | Operator::Else
        | Operator::End
        | Operator::Br { .. }
        | Operator::BrIf { .. }
        | Operator::BrTable { .. }
        | Operator::Return
        | Operator::Unreachable => compile_control_flow_operator(compiler, operator, location),
        Operator::RefIsNull
        | Operator::RefNull { .. }
        | Operator::RefFunc { .. }
        | Operator::TableFill { .. }
        | Operator::TableGet { .. }
        | Operator::TableSet { .. }
        | Operator::TableGrow { .. }
        | Operator::TableSize { .. }
        | Operator::TableCopy { .. }
        | Operator::TableInit { .. }
        | Operator::ElemDrop { .. } => compile_table_operator(compiler, operator, location),
        Operator::I64Load32U { .. }
        | Operator::I32Load { .. }
        | Operator::F32Load { .. }
        | Operator::I64Load { .. }
        | Operator::F64Load { .. }
        | Operator::I32Load8U { .. }
        | Operator::I64Load8U { .. }
        | Operator::I32Load16U { .. }
        | Operator::I64Load16U { .. }
        | Operator::I32Load8S { .. }
        | Operator::I32Load16S { .. }
        | Operator::I64Load8S { .. }
        | Operator::I64Load16S { .. }
        | Operator::I64Load32S { .. }
        | Operator::I32Store { .. }
        | Operator::F32Store { .. }
        | Operator::I64Store { .. }
        | Operator::F64Store { .. }
        | Operator::I32Store8 { .. }
        | Operator::I64Store8 { .. }
        | Operator::I32Store16 { .. }
        | Operator::I64Store16 { .. }
        | Operator::I64Store32 { .. }
        | Operator::MemorySize { .. }
        | Operator::MemoryGrow { .. } => compile_memory_operator(compiler, operator, location),
        Operator::I32Const { .. }
        | Operator::I64Const { .. }
        | Operator::F32Const { .. }
        | Operator::F64Const { .. }
        | Operator::I64Eqz
        | Operator::I32Eqz
        | Operator::I64Eq
        | Operator::I32Eq
        | Operator::I64Ne
        | Operator::I32Ne
        | Operator::I64LtS
        | Operator::I32LtS
        | Operator::I64GtS
        | Operator::I32GtS
        | Operator::I64LeS
        | Operator::I32LeS
        | Operator::I64GeS
        | Operator::I32GeS
        | Operator::I64GtU
        | Operator::I32GtU
        | Operator::I64LtU
        | Operator::I32LtU
        | Operator::I64LeU
        | Operator::I32LeU
        | Operator::I64GeU
        | Operator::I32GeU
        | Operator::F32Eq
        | Operator::F32Ne
        | Operator::F32Lt
        | Operator::F32Gt
        | Operator::F32Le
        | Operator::F32Ge
        | Operator::F64Eq
        | Operator::F64Ne
        | Operator::F64Lt
        | Operator::F64Gt
        | Operator::F64Le
        | Operator::F64Ge
        | Operator::I32Clz
        | Operator::I32Ctz
        | Operator::I32Popcnt
        | Operator::I32RemU
        | Operator::I32Rotl
        | Operator::I32Rotr
        | Operator::I64Clz
        | Operator::I64Ctz
        | Operator::I64Popcnt
        | Operator::I64Add
        | Operator::I32Add
        | Operator::I64Sub
        | Operator::I32Sub
        | Operator::I64Mul
        | Operator::I32Mul
        | Operator::I64DivS
        | Operator::I32DivS
        | Operator::I64DivU
        | Operator::I32DivU
        | Operator::I64RemS
        | Operator::I32RemS
        | Operator::I64RemU
        | Operator::I64And
        | Operator::I32And
        | Operator::I64Or
        | Operator::I32Or
        | Operator::I64Xor
        | Operator::I32Xor
        | Operator::I64Shl
        | Operator::I32Shl
        | Operator::I64ShrS
        | Operator::I32ShrS
        | Operator::I64ShrU
        | Operator::I32ShrU
        | Operator::I64Rotl
        | Operator::I64Rotr
        | Operator::F32Abs
        | Operator::F32Neg
        | Operator::F32Ceil
        | Operator::F32Floor
        | Operator::F32Trunc
        | Operator::F32Nearest
        | Operator::F32Sqrt
        | Operator::F32Add
        | Operator::F32Sub
        | Operator::F32Mul
        | Operator::F32Div
        | Operator::F32Min
        | Operator::F32Max
        | Operator::F32Copysign
        | Operator::F64Abs
        | Operator::F64Neg
        | Operator::F64Ceil
        | Operator::F64Floor
        | Operator::F64Trunc
        | Operator::F64Nearest
        | Operator::F64Sqrt
        | Operator::F64Add
        | Operator::F64Sub
        | Operator::F64Mul
        | Operator::F64Div
        | Operator::F64Min
        | Operator::F64Max
        | Operator::F64Copysign
        | Operator::I32WrapI64
        | Operator::I32TruncF32S
        | Operator::I32TruncF32U
        | Operator::I32TruncF64S
        | Operator::I32TruncF64U
        | Operator::I64ExtendI32S
        | Operator::I64ExtendI32U
        | Operator::I64TruncF32S
        | Operator::I64TruncF32U
        | Operator::I64TruncF64S
        | Operator::I64TruncF64U
        | Operator::F32ConvertI32S
        | Operator::F32ConvertI32U
        | Operator::F32ConvertI64S
        | Operator::F32ConvertI64U
        | Operator::F32DemoteF64
        | Operator::F64ConvertI32S
        | Operator::F64ConvertI32U
        | Operator::F64ConvertI64S
        | Operator::F64ConvertI64U
        | Operator::F64PromoteF32
        | Operator::I32ReinterpretF32
        | Operator::I64ReinterpretF64
        | Operator::F32ReinterpretI32
        | Operator::F64ReinterpretI64
        | Operator::I32Extend8S
        | Operator::I32Extend16S
        | Operator::I64Extend8S
        | Operator::I64Extend16S
        | Operator::I64Extend32S => compile_numeric_operator(compiler, operator, location),
        _ => Err(DFWasmError::UnsupportedOperator {
            op: format!("{operator:?}"),
            location,
        }),
    }
}

fn handle_debugger_call(compiler: &mut DFWasmCompiler, operator: &Operator, location: usize) {
    // Skip the debugger call if the option is not set
    if !compiler.options.debugger {
        return;
    }

    // Skip Nop if the option is set
    if operator == &Operator::Nop && compiler.options.skip_nop_debugger {
        return;
    }

    // Skip blocks or loops
    if matches!(operator, Operator::Block { .. } | Operator::Loop { .. }) {
        return;
    }

    let template = compiler.get_current_template();

    // Call into the debugger, with key/value information.
    template.call_function(
        "debug",
        Args::with(vec![
            string("df_function"),
            string(template.get_name().expect("No function name")),
            string("location"),
            string(format!("{location:#06x}")),
            string("next_op"),
            string(format!("{operator:?}")),
        ]),
    );
}
