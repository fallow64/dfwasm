(module $single-i64_load8_u
    (memory (export "memory") 1)

    (func $i64_load8_u (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load8_u
    )

    (export "i64_load8_u" (func $i64_load8_u))
)
