(module $single-i64_extend16_s

    (func $i64_extend16_s (param i64) (result i64)
        (local.get 0) (i64.extend16_s)
    )

    (export "i64_extend16_s" (func $i64_extend16_s))
)
