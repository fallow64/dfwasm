(module $single-i64_ge_u

    (func $i64_ge_u (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.ge_u)
    )

    (export "i64_ge_u" (func $i64_ge_u))
)
