(module $single-i64_eq

    (func $i64_eq (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.eq)
    )

    (export "i64_eq" (func $i64_eq))
)
