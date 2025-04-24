use dfwasm_template::Args;
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmResult,
    df_helper::{
        DF_FUNC_FILL_TABLE, DF_FUNC_GROW_TABLE, DF_VAR_STORE_TABLES, TemplateExt, num, var,
    },
};

pub fn compile_table_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    _location: usize,
) -> DFWasmResult<()> {
    let template = compiler.get_current_template();

    match operator {
        Operator::RefNull { hty: _ } => {
            // RefNull: Pushes a null reference to the stack.

            template.push_op_stack(num("-1"));
        }
        Operator::RefIsNull => {
            // RefIsNull: Pops a reference from the stack and pushes 1 if the reference is null, otherwise 0.

            template
                .pop_op_stack(var("$ref"))
                .compare_and_push("=", var("$ref"), num("-1"));
        }
        Operator::RefFunc { function_index } => {
            // RefFunc: Pushes a reference to a function to the stack.

            template.push_op_stack(num(function_index));
        }
        Operator::TableFill { table } => {
            // TableFill: Pops index, value, and length from the stack and fills the table with the value.
            // Similar to a memset, but for tables.
            // stack: $index, $value, $length -> ()

            template.call_function(DF_FUNC_FILL_TABLE, Args::with(vec![num(table)]));
        }
        Operator::TableGet { table } => {
            // TableGet: Pops an index from the stack and sets the value at that index in the table.

            template
                .pop_op_stack(var("$index"))
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$table"),
                        var(DF_VAR_STORE_TABLES),
                        num(table + 1),
                    ]),
                )
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$result"),
                        var(DF_VAR_STORE_TABLES),
                        num("%math(1000*%var($index)+1)"),
                    ]),
                )
                .push_op_stack(var("$result"));
        }
        Operator::TableSet { table } => {
            // TableSet: Pops an index and a value from the stack and sets the value at that index in the table.
            // stack: $index, $value -> ()

            template
                .pop_op_stack(var("$value"))
                .pop_op_stack(var("$index"))
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$table_ptr"),
                        var(DF_VAR_STORE_TABLES),
                        num(table + 1),
                    ]),
                )
                .set_var(
                    "SetListValue",
                    Args::with(vec![
                        var("%var($table_ptr)"),
                        num("%math(%var($index)+1)"),
                        var("$value"),
                    ]),
                );
        }
        Operator::TableGrow { table } => {
            // TableGrow: Pops a value from the stack, grows the specified table by that value,
            // and pushes the previous size of the table.
            // stack: $delta -> $prevSize

            // rely on function to do this because semantics can be weird, since tables are internally
            // DF lists that have a maximum value of 10,000 and that is within reason to be reached
            template.call_function(DF_FUNC_GROW_TABLE, Args::with(vec![num(table)]));
        }
        Operator::TableSize { table } => {
            // TableSize: Pushes the size of the table to the stack.

            template
                .set_var(
                    "GetListValue",
                    Args::with(vec![
                        var("$table_ptr"),
                        var(DF_VAR_STORE_TABLES),
                        num(table + 1), // indexed from 1
                    ]),
                )
                .set_var(
                    "ListLength",
                    Args::with(vec![var("$result"), var("%var($table_ptr)")]),
                )
                .set_var(
                    "/",
                    Args::with(vec![var("$result"), var("$result"), num("1000")]),
                )
                .push_op_stack(var("$result"));
        }
        Operator::TableCopy { .. } => todo!(),
        Operator::TableInit { .. } => todo!(),
        Operator::ElemDrop { .. } => todo!(),
        _ => unreachable!(),
    }

    Ok(())
}
