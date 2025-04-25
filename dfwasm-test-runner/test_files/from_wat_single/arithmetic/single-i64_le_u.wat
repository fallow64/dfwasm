(module $single-i64_le_u

    (func $i64_le_u (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.le_u)
    )

    (export "i64_le_u" (func $i64_le_u))
)
