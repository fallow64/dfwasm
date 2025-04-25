(module $single-i32_load16_u
    (memory (export "memory") 1)

    (func $i32_load16_u (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store

        i32.const 0
        i32.load16_u
    )

    (export "i32_load16_u" (func $i32_load16_u))
)
