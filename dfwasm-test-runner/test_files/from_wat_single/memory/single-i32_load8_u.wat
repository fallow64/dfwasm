(module $single-i32_load8_u
    (memory (export "memory") 1)

    (func $i32_load8_u (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store

        i32.const 0
        i32.load8_u
    )

    (export "i32_load8_u" (func $i32_load8_u))
)
