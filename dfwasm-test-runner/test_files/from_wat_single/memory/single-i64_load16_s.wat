(module $single-i64_load16_s
    (memory (export "memory") 1)

    (func $i64_load16_s (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load16_s
    )

    (export "i64_load16_s" (func $i64_load16_s))
)
