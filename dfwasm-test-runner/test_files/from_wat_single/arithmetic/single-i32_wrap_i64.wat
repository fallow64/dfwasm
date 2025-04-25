(module $single-i32_wrap_i64

    (func $i32_wrap_i64 (param i64) (result i32)
        (local.get 0) (i32.wrap_i64)
    )

    (export "i32_wrap_i64" (func $i32_wrap_i64))
)
