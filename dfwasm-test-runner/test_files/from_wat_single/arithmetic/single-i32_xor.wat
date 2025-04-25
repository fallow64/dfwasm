(module $single-i32_xor

    (func $i32_xor (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.xor)
    )

    (export "i32_xor" (func $i32_xor))
)
