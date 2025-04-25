(module $single-i64_extend8_s

    (func $i64_extend8_s (param i64) (result i64)
        (local.get 0) (i64.extend8_s)
    )

    (export "i64_extend8_s" (func $i64_extend8_s))
)
