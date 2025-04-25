(module $single-i64_store
    (memory (export "memory") 1)

    (func $i64_store (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store

        i32.const 0
        i64.load
    )

    (export "i64_store" (func $i64_store))
)
