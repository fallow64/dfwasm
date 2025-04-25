(module $single-i64_mul

    (func $i64_mul (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.mul)
    )

    (export "i64_mul" (func $i64_mul))
)
