(module $single-i64_ge_s

    (func $i64_ge_s (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.ge_s)
    )

    (export "i64_ge_s" (func $i64_ge_s))
)
