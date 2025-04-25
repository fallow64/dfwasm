(module $single-i32_load
    (memory (export "memory") 1)

    (func $i32_load (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store

        i32.const 0
        i32.load
    )

    (export "i32_load" (func $i32_load))
)
