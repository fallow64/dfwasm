(module $single-i64_extend_i32_s

    (func $i64_extend_i32_s (param i32) (result i64)
        (local.get 0) (i64.extend_i32_s)
    )

    (export "i64_extend_i32_s" (func $i64_extend_i32_s))
)
