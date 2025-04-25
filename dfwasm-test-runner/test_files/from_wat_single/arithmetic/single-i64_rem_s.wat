(module $single-i64_rem_s

    (func $i64_rem_s (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.rem_s)
    )

    (export "i64_rem_s" (func $i64_rem_s))
)
