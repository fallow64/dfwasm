use dfwasm_template::Args;
use wasmparser::Operator;

use crate::{
    DFWasmCompiler, DFWasmResult,
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
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push("=", var("$a"), var("$b"));
        }
        Operator::I64Ne | Operator::I32Ne => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push("!=", var("$a"), var("$b"));
        }
        Operator::I64LtS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push("<", var("$a"), var("$b"));
        }
        Operator::I32LtS => {
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
                .compare_and_push("<", var("$a"), var("$b"));
        }
        Operator::I64GtS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push(">", var("$a"), var("$b"));
        }
        Operator::I32GtS => {
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
                .compare_and_push(">", var("$a"), var("$b"));
        }
        Operator::I64LeS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push("<=", var("$a"), var("$b"));
        }
        Operator::I32LeS => {
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
                .compare_and_push("<=", var("$a"), var("$b"));
        }
        Operator::I64GeS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .compare_and_push(">=", var("$a"), var("$b"));
        }
        Operator::I32GeS => {
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
                .compare_and_push(">=", var("$a"), var("$b"));
        }
        Operator::I64GtU | Operator::I32GtU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "^",
                    var("$b"),
                    var("$b"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .set_var_bitwise(
                    "^",
                    var("$a"),
                    var("$a"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .compare_and_push(">", var("$a"), var("$b"));
        }
        Operator::I64LtU | Operator::I32LtU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "^",
                    var("$b"),
                    var("$b"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .set_var_bitwise(
                    "^",
                    var("$a"),
                    var("$a"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .compare_and_push("<", var("$a"), var("$b"));
        }
        Operator::I64LeU | Operator::I32LeU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "^",
                    var("$b"),
                    var("$b"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .set_var_bitwise(
                    "^",
                    var("$a"),
                    var("$a"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .compare_and_push("<=", var("$a"), var("$b"));
        }
        Operator::I64GeU | Operator::I32GeU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "^",
                    var("$b"),
                    var("$b"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .set_var_bitwise(
                    "^",
                    var("$a"),
                    var("$a"),
                    num(format_df_number_i64(i64::MIN)),
                )
                .compare_and_push(">=", var("$a"), var("$b"));
        }
        Operator::I32Clz => todo!(),
        Operator::I32Ctz => todo!(),
        Operator::I32Popcnt => todo!(),
        Operator::I64Clz => todo!(),
        Operator::I64Ctz => todo!(),
        Operator::I64Popcnt => todo!(),
        Operator::I64Add => {
            // I64Add: Pops two values from the stack and pushes their sum.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) + %var($b))"));
        }
        Operator::I32Add => {
            // I32Add: Pops two values from the stack and pushes their sum.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "&",
                    var("$res"),
                    num("%math(%var($a) + %var($b))"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                )
                .push_op_stack(var("$res"));
        }
        Operator::I64Sub => {
            // I64Sub: Pops two values from the stack and pushes their difference.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) - %var($b))"));
        }
        Operator::I32Sub => {
            // I32Sub: Pops two values from the stack and pushes their difference.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "&",
                    var("$res"),
                    num("%math(%var($a) - %var($b))"),
                    num(format_df_number_u32(0xFFFFFFFFu32)),
                )
                .push_op_stack(var("$res"));
        }
        Operator::I64Mul => {
            template.call_function(DF_FUNC_I64_MUL, Args::default());
        }
        Operator::I32Mul => {
            // I64Mul: Pops two values from the stack and pushes their product.
            // stack: $a, $b -> $result

            // We have to multiply by 1000 to get the correct DF scaling
            // 1000 * (a / 1000) * (b / 1000) = a * (b / 1000) = (a * b) / 1000

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(
                    "&",
                    var("$res"),
                    num("%math(1000 * %var($a) * %var($b))"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                )
                .push_op_stack(var("$res"));
        }
        Operator::I64DivS => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) / %var($b) / 1000)"));
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
        Operator::I32DivU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) / %var($b) / 1000)"));
        }
        Operator::I64RemS | Operator::I32RemU => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .push_op_stack(num("%math(%var($a) % %var($b))"));
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
            // .call_function(
            //     DF_FUNC_SIGN_EXTEND,
            //     Args::with(vec![
            //         num(format_df_number_usize(64)),
            //         num(format_df_number_usize(32)),
            //     ]),
            // );
        }
        Operator::I64And | Operator::I32And => {
            // I64And: Pops two values from the stack and pushes their bitwise AND.
            // stack: $a, $b -> $result

            // no need to seperate 64 and 32 bit

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I64Or | Operator::I32Or => {
            // I64Or: Pops two values from the stack and pushes their bitwise OR.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("|", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I64Xor => {
            // I64Xor: Pops two values from the stack and pushes their bitwise XOR.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("^", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I32Xor => {
            // I32Xor: Pops two values from the stack and pushes their bitwise XOR.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("^", var("$result"), var("$a"), var("$b"))
                .set_var_bitwise(
                    "&",
                    var("$result"),
                    var("$result"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                )
                .push_op_stack(var("$result"));
        }
        Operator::I64Shl => {
            // I64Shl: Pops two values from the stack and pushes the first value shifted left by the second value.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("<<", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I32Shl => {
            // I64Shl: Pops two values from the stack and pushes the first value shifted left by the second value.
            // stack: $a, $b -> $result

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
            // I64ShrS: Pops two values from the stack and pushes the first value shifted right by the second value.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(">>", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I32ShrS => {
            // I32ShrS: Pops two values from the stack and pushes the first value shifted right by the second value.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..=31)
                .set_var_bitwise(">>", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
        }
        Operator::I64ShrU => {
            // I64ShrU: Pops two values from the stack and pushes the first value shifted right by the second value.
            // stack: $a, $b -> $result

            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise(">>>", var("$result"), var("$a"), var("$b"))
                .push_op_stack(var("$result"));
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
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x3F))) // ensure $shift is 6 bits (0..=63);
                .set_var_bitwise("<<", var("$lhs"), var("$a"), var("$b"))
                .set_var_bitwise(
                    ">>>",
                    var("$rhs"),
                    var("$a"),
                    num("%math(0.064 - %var($b))"),
                )
                .set_var_bitwise("|", var("$result"), var("$lhs"), var("$rhs")) // ensure $result is 32 bits
                .push_op_stack(var("$result"));
        }
        Operator::I64Rotr => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x3F))) // ensure $shift is 6 bits (0..=63);
                .set_var_bitwise(">>>", var("$lhs"), var("$a"), var("$b"))
                .set_var_bitwise("<<", var("$rhs"), var("$a"), num("%math(0.064 - %var($b))"))
                .set_var_bitwise("|", var("$result"), var("$lhs"), var("$rhs"))
                .push_op_stack(var("$result"));
        }
        Operator::I32Rotl => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..=31);
                .set_var_bitwise("<<", var("$lhs"), var("$a"), var("$b"))
                .set_var_bitwise(
                    ">>>",
                    var("$rhs"),
                    var("$a"),
                    num("%math(0.032 - %var($b))"),
                )
                .set_var_bitwise("|", var("$result"), var("$lhs"), var("$rhs"))
                .set_var_bitwise(
                    "&",
                    var("$result"),
                    var("$result"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                ) // ensure $result is 32 bits
                .push_op_stack(var("$result"));
        }
        Operator::I32Rotr => {
            template
                .pop_op_stack(var("$b"))
                .pop_op_stack(var("$a"))
                .set_var_bitwise("&", var("$b"), var("$b"), num(format_df_number_usize(0x1F))) // ensure $shift is 5 bits (0..31);
                .set_var_bitwise(">>>", var("$lhs"), var("$a"), var("$b"))
                .set_var_bitwise("<<", var("$rhs"), var("$a"), num("%math(0.032 - %var($b))"))
                .set_var_bitwise("|", var("$result"), var("$lhs"), var("$rhs"))
                .set_var_bitwise(
                    "&",
                    var("$result"),
                    var("$result"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
                ) // ensure $result is 32 bits
                .push_op_stack(var("$result"));
        }

        Operator::I32WrapI64 => {
            template
                .pop_op_stack(var("$value"))
                .set_var_bitwise(
                    "&",
                    var("$value"),
                    var("$value"),
                    num(format_df_number_u32(0xFFFF_FFFFu32)),
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
        | Operator::F64ReinterpretI64 => todo!("floats"),
        _ => unreachable!("Invalid numeric operator: {operator:?}"),
    }

    Ok(())
}
