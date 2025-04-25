(module $single-i32_lt_u

    (func $i32_lt_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.lt_u)
    )

    (export "i32_lt_u" (func $i32_lt_u))
)
