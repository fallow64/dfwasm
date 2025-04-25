(module $single-i32_rem_u

    (func $i32_rem_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.rem_u)
    )

    (export "i32_rem_u" (func $i32_rem_u))
)
