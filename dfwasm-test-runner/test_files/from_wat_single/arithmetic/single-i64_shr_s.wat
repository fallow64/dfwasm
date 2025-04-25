(module $single-i64_shr_s

    (func $i64_shr_s (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.shr_s)
    )

    (export "i64_shr_s" (func $i64_shr_s))
)
