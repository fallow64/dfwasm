(module $single-i64_le_s

    (func $i64_le_s (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.le_s)
    )

    (export "i64_le_s" (func $i64_le_s))
)
