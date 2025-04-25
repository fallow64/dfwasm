(module $single-i64_lt_u

    (func $i64_lt_u (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.lt_u)
    )

    (export "i64_lt_u" (func $i64_lt_u))
)
