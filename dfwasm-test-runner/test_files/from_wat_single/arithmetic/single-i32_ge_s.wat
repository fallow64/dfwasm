(module $single-i32_ge_s

    (func $i32_ge_s (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.ge_s)
    )

    (export "i32_ge_s" (func $i32_ge_s))
)
