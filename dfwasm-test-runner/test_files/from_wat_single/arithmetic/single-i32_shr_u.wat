(module $single-i32_shr_u

    (func $i32_shr_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.shr_u)
    )

    (export "i32_shr_u" (func $i32_shr_u))
)
