use dfwasm_template::{Args, Block};
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmResult,
    df_helper::{
        DF_FUNC_CALL_FUNC, DF_VAR_STORE_GLOBALS, DF_VAR_STORE_TABLES, TemplateExt, get_local_name,
        num, var,
    },
};

pub fn compile_basic_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    _location: usize,
) -> DFWasmResult<()> {
    let template = compiler.get_current_template();

    match operator {
        Operator::Nop => {
            template.add_block(Block::EntityAction {
                args: Args::default(),
                action: String::new(),
                target: None,
            });
        }
        Operator::Call { function_index } => {
            // Call: Calls a function by its index.
            // Since you can import functions, we defer to DiamondFire to
            // handle finding the correct function.

            let signature = compiler.function_to_type_signature[function_index as usize].as_ref();

            let arg_count = DFWasmCompiler::arg_count_of_type(signature).expect("Not a function?");
            let result_count =
                DFWasmCompiler::result_count_of_type(signature).expect("Not a function?");

            // get the template reference again due to borrow checker
            let template = compiler.get_current_template();
            template.call_function(
                DF_FUNC_CALL_FUNC,
                Args::with(vec![
                    num(function_index.to_string()),
                    num(arg_count),
                    num(result_count),
                ]),
            );
        }
        Operator::CallIndirect {
            type_index,
            table_index,
        } => {
            // CallIndirect: Pop an index to call into a function table.

            let signature = &compiler.function_signatures[type_index as usize];
            let arg_count = DFWasmCompiler::arg_count_of_type(signature).expect("Not a function?");
            let res_count =
                DFWasmCompiler::result_count_of_type(signature).expect("Not a function?");

            // get the template reference again due to borrow checker
            let template = compiler.get_current_template();
            template
                .pop_op_stack(var("$index"))
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$table_ptr"),
                        var(DF_VAR_STORE_TABLES),
                        num(table_index + 1),
                    ]),
                )
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$func_idx"),
                        var("%var($table_ptr)"),
                        num("%math(1000*%var($index)+1)"),
                    ]),
                )
                .call_function(
                    DF_FUNC_CALL_FUNC,
                    Args::with(vec![var("$func_idx"), num(arg_count), num(res_count)]),
                );
        }
        Operator::Drop => {
            // Drop: Pops a value from the stack and does nothing.
            // i.e. Noop, but also pops a value.
            template.pop_op_stack(var("$_"));
        }
        Operator::Select | Operator::TypedSelect { .. } => {
            // Select: Pops three values from the stack and pushes one of them based on a condition.
            // stack: $if_true, $if_false, $cond -> $result

            template
                .pop_op_stack(var("$cond"))
                .pop_op_stack(var("$if_false"))
                .pop_op_stack(var("$if_true"))
                .if_var("!=", Args::with(vec![var("$cond"), num("0")]))
                .open_bracket()
                .push_op_stack(var("$if_true"))
                .close_bracket()
                .else_block()
                .open_bracket()
                .push_op_stack(var("$if_false"))
                .close_bracket();
        }
        Operator::LocalGet { local_index } => {
            // LocalGet: Gets a local variable by its index and pushes it to the stack.

            let local_var_name = get_local_name(local_index);

            template.push_op_stack(var(local_var_name));
        }
        Operator::LocalSet { local_index } => {
            // LocalSet: Pops a value from the stack and sets it to a local variable.

            let local_var_name = get_local_name(local_index);

            template
                .pop_op_stack(var("$value"))
                .set_var("=", Args::with(vec![var(local_var_name), var("$value")]));
        }
        Operator::LocalTee { local_index } => {
            // LocalTee: Pops a value from the stack and sets it to a local variable, but also
            // keeps the value on the stack

            let local_var_name = get_local_name(local_index);

            template
                .pop_op_stack(var("$tee"))
                .push_op_stack(var("$tee"))
                .set_var("=", Args::with(vec![var(local_var_name), var("$tee")]));
        }
        Operator::GlobalGet { global_index } => {
            // GlobalGet: Gets a global variable by its index and pushes it to the stack.

            // todo: make globals pointers to better allow for imports
            template
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$value"),
                        var(DF_VAR_STORE_GLOBALS),
                        num(global_index + 1),
                    ]),
                )
                .push_op_stack(var("$value"));
        }
        Operator::GlobalSet { global_index } => {
            // GlobalSet: Pops a value from the stack and sets it to a global variable.

            template.pop_op_stack(var("$value")).set_var(
                "SetListValue",
                Args::with(vec![
                    var(DF_VAR_STORE_GLOBALS),
                    num(global_index + 1),
                    var("$value"),
                ]),
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}
