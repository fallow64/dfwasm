(module $single-i32_ge_u

    (func $i32_ge_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.ge_u)
    )

    (export "i32_ge_u" (func $i32_ge_u))
)
