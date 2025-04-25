(module $single-i32_div_u

    (func $i32_div_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.div_u)
    )

    (export "i32_div_u" (func $i32_div_u))
)
