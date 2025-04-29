use dfwasm_template::{Args, Template};
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmResult,
    compiler::ControlStackEntry,
    df_helper::{
        DF_FUNC_CONTROL_LOOP_CHECK, DF_FUNC_TRAP, DF_VAR_BRANCH_COUNTER, TemplateExt,
        format_df_number_usize, generate_block_name, generate_conditional_name, generate_loop_name,
        num, string, var,
    },
};

pub fn compile_control_flow_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    location: usize,
) -> DFWasmResult<()> {
    // had to extract this out due to borrow checker :sob:
    let templates_len = compiler.templates.len();
    let template = compiler.get_current_template();

    match operator {
        Operator::Block { .. } => {
            let block_id = templates_len;
            let block_func_name =
                generate_block_name(block_id, compiler.options.module_name.as_deref());

            let template = compiler.get_current_template();
            template
                .repeat("Multiple", Args::with(vec![num("1")]))
                .open_bracket_repeat()
                .call_function(DF_FUNC_CONTROL_LOOP_CHECK, Args::default())
                .call_function(block_func_name.clone(), Args::default())
                .close_bracket_repeat()
                .call_function(DF_FUNC_CONTROL_LOOP_CHECK, Args::default());

            // push the new block template
            compiler
                .templates
                .push(Template::start_function_hidden(block_func_name));

            // push the index to the control stack
            compiler
                .control_stack
                .push(ControlStackEntry::Block(block_id));
        }
        Operator::Loop { .. } => {
            let loop_id = templates_len;
            let loop_func_name =
                generate_loop_name(loop_id, compiler.options.module_name.as_deref());

            let template = compiler.get_current_template();
            template
                .repeat("Forever", Args::default())
                .open_bracket_repeat()
                .call_function(DF_FUNC_CONTROL_LOOP_CHECK, Args::default())
                .call_function(loop_func_name.clone(), Args::default())
                .control("StopRepeat", Args::default())
                .close_bracket_repeat()
                .call_function(DF_FUNC_CONTROL_LOOP_CHECK, Args::default());

            // push the new loop template
            compiler
                .templates
                .push(Template::start_function_hidden(loop_func_name));

            // push the index to the control stack
            compiler
                .control_stack
                .push(ControlStackEntry::Loop(loop_id));
        }
        Operator::If { .. } => {
            let if_id = templates_len;
            let if_func_name =
                generate_conditional_name(if_id, compiler.options.module_name.as_deref());

            let template = compiler.get_current_template();
            template
                .pop_op_stack(var("$a"))
                .if_var("!=", Args::with(vec![var("$a"), num("0")]))
                .open_bracket()
                .call_function(if_func_name.clone(), Args::default())
                .close_bracket();

            // push the new if template
            compiler
                .templates
                .push(Template::start_function_hidden(if_func_name));

            // push the index to the control stack
            compiler.control_stack.push(ControlStackEntry::If(if_id));
        }
        Operator::Else => {
            // pop the if control stack entry
            compiler
                .control_stack
                .pop()
                .expect("Control stack is empty");

            let if_id = templates_len;
            let if_func_name =
                generate_conditional_name(if_id, compiler.options.module_name.as_deref());

            let template = compiler.get_current_template();
            template
                .else_block()
                .open_bracket()
                .call_function(if_func_name.clone(), Args::default())
                .close_bracket();

            // push the new if template
            compiler
                .templates
                .push(Template::start_function_hidden(if_func_name));

            // push the index to the control stack
            compiler.control_stack.push(ControlStackEntry::Else(if_id));
        }
        Operator::End => {
            // do nothing :)
            // If/Else: The brackets are already closed
            // Block/Loop: The brackets are already closed
            // Function: Nothing to close
            compiler
                .control_stack
                .pop()
                .expect("Control stack is empty");
        }
        Operator::Br { relative_depth } => compile_branch_operator(compiler, relative_depth),
        Operator::BrIf { relative_depth } => {
            template
                .pop_op_stack(var("$cond"))
                .if_var("!=", Args::with(vec![var("$cond"), num("0")]))
                .open_bracket();

            compile_branch_operator(compiler, relative_depth);

            let template = compiler.get_current_template();
            template.close_bracket();
        }
        Operator::BrTable { targets } => {
            template.pop_op_stack(var("$index"));

            for (i, target) in targets.targets().enumerate() {
                let target = target?;

                let template = compiler.get_current_template();
                template
                    .if_var(
                        "=",
                        Args::with(vec![var("$index"), num(format_df_number_usize(i))]),
                    )
                    .open_bracket();

                compile_branch_operator(compiler, target);

                let template = compiler.get_current_template();
                template.close_bracket();
            }

            // default target
            let default_target = targets.default();
            compile_branch_operator(compiler, default_target);
        }
        Operator::Return => {
            // count the number of blocks and loops in the control stack

            let loop_depth = compiler
                .control_stack
                .iter()
                .filter(|entry| {
                    matches!(
                        entry,
                        ControlStackEntry::Loop(_) | ControlStackEntry::Block(_)
                    )
                })
                .count();

            let template = compiler.get_current_template();

            if loop_depth == 0 {
                template.control("Skip", Args::default());
            } else {
                template
                    .set_var(
                        "=",
                        Args::with(vec![var(DF_VAR_BRANCH_COUNTER), num(loop_depth)]),
                    )
                    .control("StopRepeat", Args::default());
            }
        }
        Operator::Unreachable => {
            template.call_function(
                DF_FUNC_TRAP,
                Args::with(vec![
                    string("unreachable"),
                    string(format!("{location:#x}")),
                ]),
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn compile_branch_operator(compiler: &mut DFWasmCompiler, relative_control_stack_depth: u32) {
    // Get the control stack entry at the specified relative depth
    let control_stack_idx =
        compiler.control_stack.len() - 1 - relative_control_stack_depth as usize;

    let control_stack_entry = compiler
        .control_stack
        .get(control_stack_idx)
        .expect("Control stack entry not found");

    // Count how many blocks/loops we need to break through
    let loop_depth = compiler.control_stack[(control_stack_idx + 1)..]
        .iter()
        .filter(|entry| {
            matches!(
                entry,
                ControlStackEntry::Block(_) | ControlStackEntry::Loop(_)
            )
        })
        .count();

    match control_stack_entry {
        ControlStackEntry::Block(_) | ControlStackEntry::Loop(_) => {
            let template = compiler.get_current_template();

            if loop_depth == 0 {
                template.control("Skip", Args::default());
            } else {
                template
                    .set_var(
                        "=",
                        Args::with(vec![var(DF_VAR_BRANCH_COUNTER), num(loop_depth)]),
                    )
                    .control("StopRepeat", Args::default());
            }
        }
        _ => panic!(
            "Expected a block or loop at the relative depth, but got {control_stack_entry:?}"
        ),
    }
}
