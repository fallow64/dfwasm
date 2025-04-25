(module $single-i32_extend8_s

    (func $i32_extend8_s (param i32) (result i32)
        (local.get 0) (i32.extend8_s)
    )

    (export "i32_extend8_s" (func $i32_extend8_s))
)
