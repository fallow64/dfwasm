(module $single-i32_store16
    (memory (export "memory") 1)

    (func $i32_store16 (param $value i32) (result i32)
        i32.const 0
        local.get $value
        i32.store16

        i32.const 0
        i32.load
    )

    (export "i32_store16" (func $i32_store16))
)
