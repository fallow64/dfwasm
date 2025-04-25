(module $single-i32_sub

    (func $i32_sub (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.sub)
    )

    (export "i32_sub" (func $i32_sub))
)
