use dfwasm_template::{Args, Template};
use wasmparser::{MemArg, Operator};

use crate::{
    DFWasmCompiler, DFWasmResult,
    df_helper::{
        DF_FUNC_MEM_LOAD, DF_FUNC_MEM_STORE, DF_FUNC_SIGN_EXTEND, DF_VAR_MEM_SIZE, TemplateExt,
        format_df_number_i64, format_df_number_u64, format_df_number_usize, num, var,
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
        | Operator::F32Load { memarg } => compile_load(template, memarg, 4, None),
        Operator::I64Load { memarg } | Operator::F64Load { memarg } => {
            compile_load(template, memarg, 8, None)
        }
        Operator::I32Load8U { memarg } | Operator::I64Load8U { memarg } => {
            compile_load(template, memarg, 1, None)
        }
        Operator::I32Load16U { memarg } | Operator::I64Load16U { memarg } => {
            compile_load(template, memarg, 2, None)
        }
        Operator::I32Load8S { memarg } => compile_load(template, memarg, 1, Some(4)),
        Operator::I32Load16S { memarg } => compile_load(template, memarg, 2, Some(4)),
        Operator::I64Load8S { memarg } => compile_load(template, memarg, 1, Some(8)),
        Operator::I64Load16S { memarg } => compile_load(template, memarg, 2, Some(8)),
        Operator::I64Load32S { memarg } => compile_load(template, memarg, 4, Some(8)),
        Operator::I32Store { memarg } | Operator::F32Store { memarg } => {
            compile_store(template, memarg, 4, None)
        }
        Operator::I64Store { memarg } | Operator::F64Store { memarg } => {
            compile_store(template, memarg, 8, None)
        }
        Operator::I32Store8 { memarg } | Operator::I64Store8 { memarg } => {
            compile_store(template, memarg, 1, Some(8))
        }
        Operator::I32Store16 { memarg } | Operator::I64Store16 { memarg } => {
            compile_store(template, memarg, 2, Some(16))
        }
        Operator::I64Store32 { memarg } => compile_store(template, memarg, 4, Some(32)),
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
            // todo
        }
        Operator::MemoryInit { .. } | Operator::MemoryCopy { .. } => {
            todo!()
        }
        Operator::MemoryFill { .. } => {
            template
                .pop_op_stack(var("$n"))
                .pop_op_stack(var("$value"))
                .pop_op_stack(var("$ptr"))
                .repeat_subaction(
                    "While",
                    "!=",
                    Args::with(vec![var("$n"), num(format_df_number_i64(0))]),
                )
                .open_bracket_repeat()
                .call_function(
                    DF_FUNC_MEM_STORE,
                    Args::with(vec![
                        num("4"),
                        num("0"),
                        num("0"),
                        var("$value"),
                        var("$ptr"),
                    ]),
                )
                .set_var(
                    "+=",
                    Args::with(vec![var("$ptr"), num(format_df_number_i64(4))]),
                )
                .set_var(
                    "-=",
                    Args::with(vec![var("$n"), num(format_df_number_i64(1))]),
                )
                .close_bracket_repeat();
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn compile_load(
    template: &mut Template,
    memarg: MemArg,
    size: usize,
    sign_extend_to: Option<usize>,
) {
    // Load: Pops a memory address from the stack and loads a value from memory.
    template.call_function(
        DF_FUNC_MEM_LOAD,
        Args::with(vec![num(size), num(format_df_number_u64(memarg.offset))]),
    );

    if let Some(sign_extend_to) = sign_extend_to {
        template.call_function(
            DF_FUNC_SIGN_EXTEND,
            Args::with(vec![
                num(format_df_number_usize(size * 8)),
                num(format_df_number_usize(sign_extend_to * 8)),
            ]),
        );
    }
}

fn compile_store(template: &mut Template, memarg: MemArg, size: usize, lower_bits: Option<usize>) {
    // Store: Pops a value from the stack and a memory address, and stores the value in memory.
    // Also adds a mask argument to DF_FUNC_MEM_STORE.
    // stack: $addr, $value -> ()

    if let Some(lower_bits) = lower_bits {
        template.call_function(
            DF_FUNC_MEM_STORE,
            Args::with(vec![
                num(size),
                num(format_df_number_u64(memarg.offset)),
                num(format_df_number_usize((1 << lower_bits) - 1)),
            ]),
        );
    } else {
        template.call_function(
            DF_FUNC_MEM_STORE,
            Args::with(vec![num(size), num(format_df_number_u64(memarg.offset))]),
        );
    }
}
