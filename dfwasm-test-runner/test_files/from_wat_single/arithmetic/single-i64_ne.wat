(module $single-i64_ne

    (func $i64_ne (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.ne)
    )

    (export "i64_ne" (func $i64_ne))
)
