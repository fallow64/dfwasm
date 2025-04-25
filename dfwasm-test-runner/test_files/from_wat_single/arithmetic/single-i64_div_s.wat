(module $single-i64_div_s

    (func $i64_div_s (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.div_s)
    )

    (export "i64_div_s" (func $i64_div_s))
)
