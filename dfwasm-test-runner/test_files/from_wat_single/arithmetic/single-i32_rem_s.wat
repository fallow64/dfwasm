(module $single-i32_rem_s

    (func $i32_rem_s (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.rem_s)
    )

    (export "i32_rem_s" (func $i32_rem_s))
)
