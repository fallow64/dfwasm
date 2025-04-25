(module $single-i64_load16_u
    (memory (export "memory") 1)

    (func $i64_load16_u (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load16_u
    )

    (export "i64_load16_u" (func $i64_load16_u))
)
