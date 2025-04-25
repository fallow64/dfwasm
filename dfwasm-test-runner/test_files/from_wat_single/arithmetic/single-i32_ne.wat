(module $single-i32_ne

    (func $i32_ne (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.ne)
    )

    (export "i32_ne" (func $i32_ne))
)
