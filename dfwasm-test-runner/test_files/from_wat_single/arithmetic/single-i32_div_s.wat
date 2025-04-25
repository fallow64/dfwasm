(module $single-i32_div_s

    (func $i32_div_s (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.div_s)
    )

    (export "i32_div_s" (func $i32_div_s))
)
