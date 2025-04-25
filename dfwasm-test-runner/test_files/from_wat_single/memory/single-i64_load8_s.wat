(module $single-i64_load8_s
    (memory (export "memory") 1)

    (func $i64_load8_s (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load8_s
    )

    (export "i64_load8_s" (func $i64_load8_s))
)
