(module $single-i32_and

    (func $i32_and (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.and)
    )

    (export "i32_and" (func $i32_and))
)
