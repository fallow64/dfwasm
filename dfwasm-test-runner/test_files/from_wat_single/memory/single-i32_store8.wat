(module $single-i32_store8
    (memory (export "memory") 1)

    (func $i32_store8 (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store8

        i32.const 0
        i32.load
    )

    (export "i32_store8" (func $i32_store8))
)
