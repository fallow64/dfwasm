(module $single-i32_ctz

    (func $i32_ctz (param i32) (result i32)
        (local.get 0) (i32.ctz)
    )

    (export "i32_ctz" (func $i32_ctz))
)
