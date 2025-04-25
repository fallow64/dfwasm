(module $single-i64_store8
    (memory (export "memory") 1)

    (func $i64_store8 (param $value i64) (result i64)
        i32.const 0
        local.get $value
        i64.store8

        i32.const 0
        i64.load
    )

    (export "i64_store8" (func $i64_store8))
)
