(module $single-i64_lt_s

    (func $i64_lt_s (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.lt_s)
    )

    (export "i64_lt_s" (func $i64_lt_s))
)
