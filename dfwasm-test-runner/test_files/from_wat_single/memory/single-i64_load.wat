(module $single-i64_load
    (memory (export "memory") 1)

    (func $i64_load (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load
    )

    (export "i64_load" (func $i64_load))
)
