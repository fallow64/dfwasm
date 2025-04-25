(module $single-i32_rotr

    (func $i32_rotr (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.rotr)
    )

    (export "i32_rotr" (func $i32_rotr))
)
