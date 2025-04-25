(module $single-i32_extend16_s

    (func $i32_extend16_s (param i32) (result i32)
        (local.get 0) (i32.extend16_s)
    )

    (export "i32_extend16_s" (func $i32_extend16_s))
)
