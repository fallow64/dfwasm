(module $single-i64_and

    (func $i64_and (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.and)
    )

    (export "i64_and" (func $i64_and))
)
