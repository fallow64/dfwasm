(module $single-i32_shr_s

    (func $i32_shr_s (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.shr_s)
    )

    (export "i32_shr_s" (func $i32_shr_s))
)
