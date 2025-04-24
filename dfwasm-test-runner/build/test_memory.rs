use crate::{Type, clear_directory, numbers::I32_1, write_test};

enum MemoryTestType {
    Store,
    Load,
}

fn build_memory_instruction_test(
    instr: &str,
    test_type: MemoryTestType,
    value_type: Type,
    tests: &[&[&str]],
) {
    let function_name = instr.replace(".", "_");
    let module_name = format!("single-{}", function_name);

    let load_instruction = match test_type {
        MemoryTestType::Load => instr,
        MemoryTestType::Store => match value_type {
            Type::I32 => "i32.load",
            Type::I64 => "i64.load",
        },
    };

    let store_instruction = match test_type {
        MemoryTestType::Load => match value_type {
            Type::I32 => "i32.store",
            Type::I64 => "i64.store",
        },
        MemoryTestType::Store => instr,
    };

    let wat_file = format!(
        r#"(module ${module_name}
    (memory (export "memory") 1)

    (func ${function_name} (param $value {value_type}) (result {value_type})
        i32.const 0
        local.get $value
        {store_instruction}

        i32.const 0
        {load_instruction}
    )

    (export "{function_name}" (func ${function_name}))
)"#
    );

    let mut test_file = String::new();
    for test in tests {
        let call_args = test
            .iter()
            .take(1)
            .map(|arg| *arg)
            .collect::<Vec<_>>()
            .join(" ");

        test_file.push_str(&format!("{} {}\n", function_name, call_args));
    }

    write_test("wat_single", "memory", &module_name, &wat_file, &test_file);
}

pub fn build() {
    clear_directory("wat_single", "memory");

    let load_instructions = vec![
        "i32.load",
        "i64.load",
        "i32.load8_u",
        "i32.load8_s",
        "i32.load16_u",
        "i32.load16_s",
        "i64.load8_u",
        "i64.load8_s",
        "i64.load16_u",
        "i64.load16_s",
        "i64.load32_u",
        "i64.load32_s",
    ];

    let store_instructions = vec![
        "i32.store",
        "i64.store",
        "i32.store8",
        "i32.store16",
        "i64.store8",
        "i64.store16",
        "i64.store32",
    ];

    for load_instruction in load_instructions {
        build_memory_instruction_test(
            load_instruction,
            MemoryTestType::Load,
            if load_instruction.contains("i32") {
                Type::I32
            } else {
                Type::I64
            },
            I32_1,
        );
    }

    for store_instruction in store_instructions {
        build_memory_instruction_test(
            store_instruction,
            MemoryTestType::Store,
            if store_instruction.contains("i32") {
                Type::I32
            } else {
                Type::I64
            },
            I32_1,
        );
    }
}
