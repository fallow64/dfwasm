use core::panic;

use crate::{
    Type,
    numbers::{I32_1, I32_2, I64_1, I64_2},
    write_test,
};
use Type::*;

type Instruction = (&'static str, &'static [Type], &'static [Type]);
type TestCase = &'static [&'static str];

fn build_instruction_test(instruction: &Instruction, tests: &[TestCase]) {
    let function_name = instruction.0.replace(".", "_");
    let module_name = format!("single-{}", function_name);

    let instr_asm = instruction.0;
    let instr_args = instruction.1;
    let instr_res = instruction.2;

    let function_body = match instr_args.len() {
        2 => format!("(local.get 0) (local.get 1) ({instr_asm})"),
        1 => format!("(local.get 0) (local.get 1) ({instr_asm})"),
        _ => panic!("Invalid number of arguments for instruction: {instr_asm}"),
    };

    let params = instr_args
        .iter()
        .map(|t| format!("{t}"))
        .collect::<Vec<_>>()
        .join(" ");
    let results = instr_res
        .iter()
        .map(|t| format!("{t}"))
        .collect::<Vec<_>>()
        .join(" ");

    let signature = format!("(params {params}) (result {results})");
    let wat_file = format!(
        r#"(module ${module_name}

    (func ${function_name} {signature}
        {function_body}
    )

    (export "{function_name}" (func ${function_name}))
)"#
    );

    let mut test_file = String::new();
    for test in tests {
        let call_args = test
            .iter()
            .take(instr_args.len())
            .map(|arg| *arg)
            .collect::<Vec<_>>()
            .join(" ");

        test_file.push_str(&format!("{} {}", function_name, call_args));
    }

    write_test(
        "wat_single",
        "arithmetic",
        &module_name,
        &wat_file,
        &test_file,
    );
}

fn build_i32() {
    let i32_instructions: Vec<Instruction> = vec![
        ("i32.add", &[I32, I32], &[I32]),
        ("i32.sub", &[I32, I32], &[I32]),
        ("i32.mul", &[I32, I32], &[I32]),
        ("i32.div_s", &[I32, I32], &[I32]),
        ("i32.div_u", &[I32, I32], &[I32]),
        ("i32.rem_s", &[I32, I32], &[I32]),
        ("i32.rem_u", &[I32, I32], &[I32]),
        ("i32.and", &[I32, I32], &[I32]),
        ("i32.or", &[I32, I32], &[I32]),
        ("i32.xor", &[I32, I32], &[I32]),
        ("i32.shl", &[I32, I32], &[I32]),
        ("i32.shr_s", &[I32, I32], &[I32]),
        ("i32.shr_u", &[I32, I32], &[I32]),
        ("i32.rotl", &[I32, I32], &[I32]),
        ("i32.rotr", &[I32, I32], &[I32]),
        ("i32.clz", &[I32], &[I32]),
        ("i32.ctz", &[I32], &[I32]),
        ("i32.popcnt", &[I32], &[I32]),
        ("i32.eqz", &[I32], &[I32]),
        ("i32.eq", &[I32, I32], &[I32]),
        ("i32.ne", &[I32, I32], &[I32]),
        ("i32.lt_s", &[I32, I32], &[I32]),
        ("i32.lt_u", &[I32, I32], &[I32]),
        ("i32.gt_s", &[I32, I32], &[I32]),
        ("i32.gt_u", &[I32, I32], &[I32]),
        ("i32.le_s", &[I32, I32], &[I32]),
        ("i32.le_u", &[I32, I32], &[I32]),
        ("i32.ge_s", &[I32, I32], &[I32]),
        ("i32.ge_u", &[I32, I32], &[I32]),
        ("i64.extend_i32_s", &[I32], &[I64]),
        ("i64.extend_i32_u", &[I32], &[I64]),
        ("i32.extend8_s", &[I32], &[I32]),
        ("i32.extend16_s", &[I32], &[I32]),
    ];

    for instruction in i32_instructions {
        let tests = match instruction.1.len() {
            2 => I32_2,
            1 => I32_1,
            _ => panic!(
                "Invalid number of arguments for instruction: {}",
                instruction.0
            ),
        };
        build_instruction_test(&instruction, I32_1);
    }
}

fn build_i64() {
    let i64_instructions: Vec<Instruction> = vec![
        ("i64.add", &[I64, I64], &[I64]),
        ("i64.sub", &[I64, I64], &[I64]),
        ("i64.mul", &[I64, I64], &[I64]),
        ("i64.div_s", &[I64, I64], &[I64]),
        ("i64.div_u", &[I64, I64], &[I64]),
        ("i64.rem_s", &[I64, I64], &[I64]),
        ("i64.rem_u", &[I64, I64], &[I64]),
        ("i64.and", &[I64, I64], &[I64]),
        ("i64.or", &[I64, I64], &[I64]),
        ("i64.xor", &[I64, I64], &[I64]),
        ("i64.shl", &[I64, I64], &[I64]),
        ("i64.shr_s", &[I64, I64], &[I64]),
        ("i64.shr_u", &[I64, I64], &[I64]),
        ("i64.rotl", &[I64, I64], &[I64]),
        ("i64.rotr", &[I64, I64], &[I64]),
        ("i64.clz", &[I64], &[I64]),
        ("i64.ctz", &[I64], &[I64]),
        ("i64.popcnt", &[I64], &[I64]),
        ("i64.eqz", &[I64], &[I32]),
        ("i64.eq", &[I64, I64], &[I64]),
        ("i64.ne", &[I64, I64], &[I64]),
        ("i64.lt_s", &[I64, I64], &[I64]),
        ("i64.lt_u", &[I64, I64], &[I64]),
        ("i64.gt_s", &[I64, I64], &[I64]),
        ("i64.gt_u", &[I64, I64], &[I64]),
        ("i64.le_s", &[I64, I64], &[I64]),
        ("i64.le_u", &[I64, I64], &[I64]),
        ("i64.ge_s", &[I64, I64], &[I64]),
        ("i64.ge_u", &[I64, I64], &[I64]),
        ("i32.wrap_i64", &[I64], &[I32]),
        ("i64.extend8_s", &[I64], &[I64]),
        ("i64.extend16_s", &[I64], &[I64]),
        ("i64.extend32_s", &[I64], &[I64]),
    ];

    for instruction in i64_instructions {
        let tests = match instruction.1.len() {
            2 => I64_2,
            1 => I64_1,
            _ => panic!(
                "Invalid number of arguments for instruction: {}",
                instruction.0
            ),
        };
        build_instruction_test(&instruction, tests);
    }
}

pub fn build() {
    build_i32();
    build_i64();
}
