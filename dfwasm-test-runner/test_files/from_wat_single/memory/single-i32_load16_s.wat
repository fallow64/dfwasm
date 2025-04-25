(module $single-i32_load16_s
    (memory (export "memory") 1)

    (func $i32_load16_s (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store

        i32.const 0
        i32.load16_s
    )

    (export "i32_load16_s" (func $i32_load16_s))
)
