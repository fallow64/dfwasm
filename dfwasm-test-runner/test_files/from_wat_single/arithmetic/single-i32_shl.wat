(module $single-i32_shl

    (func $i32_shl (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.shl)
    )

    (export "i32_shl" (func $i32_shl))
)
