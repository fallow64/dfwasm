(module $single-i32_add

    (func $i32_add (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.add)
    )

    (export "i32_add" (func $i32_add))
)
