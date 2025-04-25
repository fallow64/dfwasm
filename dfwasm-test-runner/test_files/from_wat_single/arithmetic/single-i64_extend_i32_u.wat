(module $single-i64_extend_i32_u

    (func $i64_extend_i32_u (param i32) (result i64)
        (local.get 0) (i64.extend_i32_u)
    )

    (export "i64_extend_i32_u" (func $i64_extend_i32_u))
)
