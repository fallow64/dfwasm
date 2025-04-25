use dfwasm_template::Args;
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmResult,
    df_helper::{
        DF_FUNC_MEM_LOAD, DF_FUNC_MEM_STORE, DF_FUNC_SIGN_EXTEND, DF_VAR_MEM_SIZE, TemplateExt,
        format_df_number_i64, format_df_number_u64, num, var,
    },
};

pub fn compile_memory_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    _location: usize,
) -> DFWasmResult<()> {
    let template = compiler.get_current_template();

    match operator {
        Operator::I64Load32U { memarg }
        | Operator::I32Load { memarg }
        | Operator::F32Load { memarg } => {
            // Load: Pops a memory address from the stack and loads a value from memory.
            template.call_function(
                DF_FUNC_MEM_LOAD,
                Args::with(vec![num("4"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I64Load { memarg } | Operator::F64Load { memarg } => {
            // Load: Pops a memory address from the stack and loads a value from memory.
            template.call_function(
                DF_FUNC_MEM_LOAD,
                Args::with(vec![num("8"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I32Load8U { memarg } | Operator::I64Load8U { memarg } => {
            // Load8U: Pops a memory address from the stack and loads a 1-byte value from memory.
            template.call_function(
                DF_FUNC_MEM_LOAD,
                Args::with(vec![num("1"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I32Load16U { memarg } | Operator::I64Load16U { memarg } => {
            // Load16U: Pops a memory address from the stack and loads a 2-byte value from memory.
            template.call_function(
                DF_FUNC_MEM_LOAD,
                Args::with(vec![num("2"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I32Load8S { memarg } => {
            // Load8S: Pops a memory address from the stack and loads a 1-byte sign extended value from memory.
            template
                .call_function(
                    DF_FUNC_MEM_LOAD,
                    Args::with(vec![num("1"), num(format_df_number_u64(memarg.offset))]),
                )
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_u64(8)),
                        num(format_df_number_u64(32)),
                    ]),
                );
        }
        Operator::I32Load16S { memarg } => {
            // Load16S: Pops a memory address from the stack and loads a 2-byte sign extended value from memory.
            template
                .call_function(
                    DF_FUNC_MEM_LOAD,
                    Args::with(vec![num("2"), num(format_df_number_u64(memarg.offset))]),
                )
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_u64(16)),
                        num(format_df_number_u64(32)),
                    ]),
                );
        }
        Operator::I64Load8S { memarg } => {
            // Load8S: Pops a memory address from the stack and loads a 1-byte sign extended value from memory.
            template
                .call_function(
                    DF_FUNC_MEM_LOAD,
                    Args::with(vec![num("1"), num(format_df_number_u64(memarg.offset))]),
                )
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_u64(8)),
                        num(format_df_number_u64(64)),
                    ]),
                );
        }
        Operator::I64Load16S { memarg } => {
            // Load16S: Pops a memory address from the stack and loads a 2-byte sign extended value from memory.
            template
                .call_function(
                    DF_FUNC_MEM_LOAD,
                    Args::with(vec![num("2"), num(format_df_number_u64(memarg.offset))]),
                )
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_u64(16)),
                        num(format_df_number_u64(64)),
                    ]),
                );
        }
        Operator::I64Load32S { memarg } => {
            // Load32S: Pops a memory address from the stack and loads a 4-byte sign extended value from memory.
            template
                .call_function(
                    DF_FUNC_MEM_LOAD,
                    Args::with(vec![num("4"), num(format_df_number_u64(memarg.offset))]),
                )
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_u64(32)),
                        num(format_df_number_u64(64)),
                    ]),
                );
        }
        Operator::I32Store { memarg } | Operator::F32Store { memarg } => {
            // Store: Pops a value from the stack and a memory address, and stores the value in memory.
            // stack: $addr, $value -> ()

            template.call_function(
                DF_FUNC_MEM_STORE,
                Args::with(vec![num("4"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I64Store { memarg } | Operator::F64Store { memarg } => {
            // Store: Pops a value from the stack and a memory address, and stores the value in memory.
            // stack: $addr, $value -> ()

            template.call_function(
                DF_FUNC_MEM_STORE,
                Args::with(vec![num("8"), num(format_df_number_u64(memarg.offset))]),
            );
        }
        Operator::I32Store8 { memarg } | Operator::I64Store8 { memarg } => {
            // Store: Pops a value from the stack and a memory address, and stores the value in memory.
            // Also adds a mask argument to DF_FUNC_MEM_STORE.
            // stack: $addr, $value -> ()

            template.call_function(
                DF_FUNC_MEM_STORE,
                Args::with(vec![
                    num("1"),
                    num(format_df_number_u64(memarg.offset)),
                    num(format_df_number_i64(0xFF)),
                ]),
            );
        }
        Operator::I32Store16 { memarg } | Operator::I64Store16 { memarg } => {
            // Store: Pops a value from the stack and a memory address, and stores the value in memory.
            // Also adds a mask argument to DF_FUNC_MEM_STORE.
            // stack: $addr, $value -> ()

            template.call_function(
                DF_FUNC_MEM_STORE,
                Args::with(vec![
                    num("2"),
                    num(format_df_number_u64(memarg.offset)),
                    num(format_df_number_i64(0xFFFF)),
                ]),
            );
        }
        Operator::I64Store32 { memarg } => {
            // Store: Pops a value from the stack and a memory address, and stores the value in memory.
            // Also adds a mask argument to DF_FUNC_MEM_STORE.
            // stack: $addr, $value -> ()

            template.call_function(
                DF_FUNC_MEM_STORE,
                Args::with(vec![
                    num("4"),
                    num(format_df_number_u64(memarg.offset)),
                    num(format_df_number_i64(0xFFFFFFFF)),
                ]),
            );
        }
        Operator::MemorySize { mem: _ } => {
            // MemorySize: Gets the size of the memory and pushes it to the stack
            template.push_op_stack(var(DF_VAR_MEM_SIZE));
        }
        Operator::MemoryGrow { mem: _ } => {
            // MemoryGrow: Pops a value from the stack, grows the memory by that value,
            // and pushes the previous memory size.

            template
                .pop_op_stack(var("$delta"))
                .push_op_stack(var(DF_VAR_MEM_SIZE))
                .set_var("+=", Args::with(vec![var(DF_VAR_MEM_SIZE), var("$delta")]));
        }
        Operator::DataDrop { .. } => {
            // DataDrop: Drops a data segment from the memory.
            // (nop in DF)
        }
        Operator::MemoryInit { .. } | Operator::MemoryCopy { .. } | Operator::MemoryFill { .. } => {
            todo!()
        }
        _ => unreachable!(),
    }

    Ok(())
}
