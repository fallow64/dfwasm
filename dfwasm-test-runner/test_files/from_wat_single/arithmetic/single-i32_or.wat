(module $single-i32_or

    (func $i32_or (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.or)
    )

    (export "i32_or" (func $i32_or))
)
