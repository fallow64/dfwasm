(module $single-i64_or

    (func $i64_or (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.or)
    )

    (export "i64_or" (func $i64_or))
)
