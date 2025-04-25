(module $single-i64_extend32_s

    (func $i64_extend32_s (param i64) (result i64)
        (local.get 0) (i64.extend32_s)
    )

    (export "i64_extend32_s" (func $i64_extend32_s))
)
