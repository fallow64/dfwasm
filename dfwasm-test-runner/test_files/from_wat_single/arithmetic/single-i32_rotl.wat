(module $single-i32_rotl

    (func $i32_rotl (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.rotl)
    )

    (export "i32_rotl" (func $i32_rotl))
)
