(module $single-i32_mul

    (func $i32_mul (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.mul)
    )

    (export "i32_mul" (func $i32_mul))
)
