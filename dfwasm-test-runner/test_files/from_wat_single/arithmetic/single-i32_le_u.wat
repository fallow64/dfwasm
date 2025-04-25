(module $single-i32_le_u

    (func $i32_le_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.le_u)
    )

    (export "i32_le_u" (func $i32_le_u))
)
