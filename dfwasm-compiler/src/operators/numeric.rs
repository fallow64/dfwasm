use dfwasm_template::{Args, Template};
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmError, DFWasmResult,
    df_helper::{
        DF_FUNC_I64_MUL, DF_FUNC_SIGN_EXTEND, TemplateExt, format_df_number_i32,
        format_df_number_i64, format_df_number_u32, format_df_number_u64, format_df_number_usize,
        num, var,
    },
};

pub fn compile_numeric_operator(
    compiler: &mut DFWasmCompiler,
    operator: Operator,
    _location: usize,
) -> DFWasmResult<()> {
    let template = compiler.get_current_template();

    match operator {
        Operator::I32Const { value } => {
            let formatted = format_df_number_i32(value);
            template.push_op_stack(num(formatted));
        }
        Operator::I64Const { value } => {
            let formatted = format_df_number_i64(value);
            template.push_op_stack(num(formatted));
        }
        Operator::I64Eqz | Operator::I32Eqz => {
            template
                .pop_op_stack(var("$value"))
                .compare_and_push("=", var("$value"), num("0"));
        }
        Operator::I64Eq | Operator::I32Eq => {
            compile_comparison_operator(template, "=", false, false);
        }
        Operator::I64Ne | Operator::I32Ne => {
            compile_comparison_operator(template, "!=", false, false);
        }
        Operator::I64LtS => {
            compile_comparison_operator(template, "<", false, false);
        }
        Operator::I32LtS => {
            compile_comparison_operator(template, "<", true, false);
        }
        Operator::I64GtS => {
            compile_comparison_operator(template, ">", false, false);
        }
        Operator::I32GtS => {
            compile_comparison_operator(template, ">", true, false);
        }
        Operator::I64LeS => {
            compile_comparison_operator(template, "<=", false, false);
        }
        Operator::I32LeS => {
            compile_comparison_operator(template, "<=", true, false);
        }
        Operator::I64GeS => {
            compile_comparison_operator(template, ">=", false, false);
        }
        Operator::I32GeS => {
            compile_comparison_operator(template, ">=", true, false);
        }
        Operator::I64GtU | Operator::I32GtU => {
            compile_comparison_operator(template, ">", false, true);
        }
        Operator::I64LtU | Operator::I32LtU => {
            compile_comparison_operator(template, "<", false, true);
        }
        Operator::I64LeU | Operator::I32LeU => {
            compile_comparison_operator(template, "<=", false, true);
        }
        Operator::I64GeU | Operator::I32GeU => {
            compile_comparison_operator(template, ">=", false, true);
        }
        Operator::I64Clz | Operator::I32Clz => todo!(),
        Operator::I64Ctz | Operator::I32Ctz => todo!(),
        Operator::I64Popcnt | Operator::I32Popcnt => todo!(),
        Operator::I64Add => {
            compile_binary_operator(template, "%math(%var($a)+%var($b))", None);
        }
        Operator::I32Add => {
            compile_binary_operator(template, "%math(%var($a)+%var($b))", Some(u32::MAX));
        }
        Operator::I64Sub => {
            compile_binary_operator(template, "%math(%var($a)-%var($b))", None);
        }
        Operator::I32Sub => {
            compile_binary_operator(template, "%math(%var($a)-%var($b))", Some(u32::MAX));
        }
        Operator::I64Mul => {
            // Due to precision issues, i64 multiplication in DF is not to spec.
            // Therefore, we use a custom multiplication handler.
            template.call_function(DF_FUNC_I64_MUL, Args::default());
        }
        Operator::I32Mul => {
            compile_binary_operator(template, "%math(1000*%var($a)*%var($b))", Some(u32::MAX));
        }
        Operator::I64DivS | Operator::I32DivU => {
            compile_binary_operator(template, "%math(%var($a) / %var($b) / 1000)", None);
        }
        Operator::I32DivS => {
            template
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_usize(32)),
                        num(format_df_number_usize(64)),
                    ]),
                )
                .pop_op_stack(var("$b"))
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_usize(32)),
                        num(format_df_number_usize(64)),
                    ]),
                )
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "&",
                    var("$res"),
                    num("%math(%var($a) / %var($b) / 1000)"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                )
                .push_op_stack(var("$res"));
        }
        Operator::I64DivU => {
            //todo: 64bit unsigned
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) / %var($b) / 1000)"));
        }
        Operator::I64RemS | Operator::I32RemU => {
            compile_binary_operator(template, "%math(%var($a) % %var($b))", None);
        }
        Operator::I32RemS => {
            template
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_usize(32)),
                        num(format_df_number_usize(32)),
                    ]),
                )
                .pop_op_stack(var("$b"))
                .call_function(
                    DF_FUNC_SIGN_EXTEND,
                    Args::with(vec![
                        num(format_df_number_usize(32)),
                        num(format_df_number_usize(32)),
                    ]),
                )
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) % %var($b))"));
        }
        Operator::I64And | Operator::I32And => {
            compile_bitwise_operator(template, "&", None);
        }
        Operator::I64Or | Operator::I32Or => {
            compile_bitwise_operator(template, "|", None);
        }
        Operator::I64Xor => {
            compile_bitwise_operator(template, "^", None);
        }
        Operator::I32Xor => {
            compile_bitwise_operator(template, "^", Some(u32::MAX));
        }
        Operator::I64Shl => {
            compile_bitwise_operator(template, "<<", None);
        }
        Operator::I32Shl => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..=31)
                .set_var_bitwise("<<", var("$result"), var("$a"), var("$b"))
                .set_var_bitwise(
                    "&",
                    var("$result"),
                    var("$result"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                )
                .push_op_stack(var("$result"));
        }
        Operator::I64ShrS => {
            compile_bitwise_operator(template, ">>", None);
        }
        Operator::I32ShrS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..=31)
                .set_var_bitwise(">>", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I64ShrU => {
            compile_bitwise_operator(template, ">>>", None);
        }
        Operator::I32ShrU => {
            // I32ShrU: Pops two values from the stack and pushes the first value shifted right by the second value.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..=31)
                .set_var_bitwise(">>>", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I64Rotl => {
            compile_rotate_operator(template, 64, false);
        }
        Operator::I64Rotr => {
            compile_rotate_operator(template, 64, true);
        }
        Operator::I32Rotl => {
            compile_rotate_operator(template, 32, false);
        }
        Operator::I32Rotr => {
            compile_rotate_operator(template, 32, true);
        }
        Operator::I32WrapI64 => {
            template
                .pop_op_stack(var("$value"))
                .set_var_bitwise(
                    "&",
                    var("$value"),
                    var("$value"),
                    num(format_df_number_u32(u32::MAX)),
                )
                .push_op_stack(var("$value"));
        }
        Operator::I64ExtendI32S => {
            // I64ExtendI32S: Pops a value from the stack and sign-extends it to 64 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(32)),
                    num(format_df_number_u64(64)),
                ]),
            );
        }
        Operator::I64ExtendI32U => {} // essentially a no-op
        Operator::I32Extend8S => {
            // I32Extend8S: Pops a value from the stack and sign-extends it to 32 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(8)),
                    num(format_df_number_u64(32)),
                ]),
            );
        }
        Operator::I32Extend16S => {
            // I32Extend16S: Pops a value from the stack and sign-extends it to 32 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(16)),
                    num(format_df_number_u64(32)),
                ]),
            );
        }
        Operator::I64Extend8S => {
            // I64Extend8S: Pops a value from the stack and sign-extends it to 64 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(8)),
                    num(format_df_number_u64(64)),
                ]),
            );
        }
        Operator::I64Extend16S => {
            // I64Extend16S: Pops a value from the stack and sign-extends it to 64 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(16)),
                    num(format_df_number_u64(64)),
                ]),
            );
        }
        Operator::I64Extend32S => {
            // I64Extend32S: Pops a value from the stack and sign-extends it to 64 bits.
            // stack: $value -> $result

            template.call_function(
                DF_FUNC_SIGN_EXTEND,
                Args::with(vec![
                    num(format_df_number_u64(32)),
                    num(format_df_number_u64(64)),
                ]),
            );
        }
        Operator::F32Const { .. }
        | Operator::F64Const { .. }
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
        | Operator::I32TruncF32S
        | Operator::I32TruncF32U
        | Operator::I32TruncF64S
        | Operator::I32TruncF64U
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
        | Operator::F64ReinterpretI64 => {
            return Err(DFWasmError::NotYetImplemented(
                "floats are not supported yet",
            ));
        }
        _ => unreachable!("Invalid numeric operator: {operator:?}"),
    }

    Ok(())
}

fn compile_rotate_operator(template: &mut Template, bits: usize, is_right_shift: bool) {
    let shift_mask = match bits {
        32 => 0x1Fusize,
        64 => 0x3Fusize,
        _ => unreachable!("Invalid bit size: {bits}"),
    };

    template
        .pop_op_stack(var("$b"))
        .pop_op_stack(var("$a"))
        .set_var_bitwise(
            "&",
            var("$b"),
            var("$b"),
            num(format_df_number_usize(shift_mask)),
        ); // clamp $shift to 5 bits (0..=31) or 6 bits (0..=63)

    if is_right_shift {
        template
            .set_var_bitwise(">>>", var("$lhs"), var("$a"), var("$b"))
            .set_var_bitwise(
                "<<",
                var("$rhs"),
                var("$a"),
                num(format!("%math(0.0{bits} - %var($b))")),
            );
    } else {
        template
            .set_var_bitwise("<<", var("$lhs"), var("$a"), var("$b"))
            .set_var_bitwise(
                ">>>",
                var("$rhs"),
                var("$a"),
                num(format!("%math(0.0{bits} - %var($b))")),
            );
    }

    template.set_var_bitwise("|", var("$result"), var("$lhs"), var("$rhs"));
    if bits == 32 {
        template.set_var_bitwise(
            "&",
            var("$result"),
            var("$result"),
            num(format_df_number_usize(0xFFFF_FFFFusize)),
        ); // ensure $result is 32 bits
    }
    template.push_op_stack(var("$result"));
}

fn compile_comparison_operator(
    template: &mut Template,
    action: &str,
    sign_extend: bool,
    xor: bool,
) {
    if sign_extend {
        template.call_function(
            DF_FUNC_SIGN_EXTEND,
            Args::with(vec![
                num(format_df_number_usize(32)),
                num(format_df_number_usize(64)),
            ]),
        );
    }
    template.pop_op_stack(var("$b"));

    if sign_extend {
        template.call_function(
            DF_FUNC_SIGN_EXTEND,
            Args::with(vec![
                num(format_df_number_usize(32)),
                num(format_df_number_usize(64)),
            ]),
        );
    }
    template.pop_op_stack(var("$a"));

    if xor {
        template.set_var_bitwise(
            "^",
            var("$a"),
            var("$a"),
            num(format_df_number_i64(i64::MIN)),
        );
        template.set_var_bitwise(
            "^",
            var("$b"),
            var("$b"),
            num(format_df_number_i64(i64::MIN)),
        );
    }

    template.compare_and_push(action, var("$a"), var("$b"));
}

fn compile_binary_operator(template: &mut Template, expr: &str, mask: Option<u32>) {
    template.pop_op_stack(var("$b")).pop_op_stack(var("$a"));

    let value = num(expr);

    if let Some(mask) = mask {
        template.set_var_bitwise("&", var("$res"), value, num(format_df_number_u32(mask)));
        template.push_op_stack(var("$res"));
    } else {
        template.push_op_stack(value);
    }
}

fn compile_bitwise_operator(template: &mut Template, operator: &str, mask: Option<u32>) {
    template
        .pop_op_stack(var("$b"))
        .pop_op_stack(var("$a"))
        .set_var_bitwise(operator, var("$res"), var("$a"), var("$b"));

    if let Some(mask) = mask {
        template.set_var_bitwise(
            "&",
            var("$res"),
            var("$res"),
            num(format_df_number_u32(mask)),
        );
    }

    template.push_op_stack(var("$res"));
}
