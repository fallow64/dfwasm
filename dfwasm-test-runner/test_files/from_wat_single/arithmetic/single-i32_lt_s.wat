(module $single-i32_lt_s

    (func $i32_lt_s (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.lt_s)
    )

    (export "i32_lt_s" (func $i32_lt_s))
)
