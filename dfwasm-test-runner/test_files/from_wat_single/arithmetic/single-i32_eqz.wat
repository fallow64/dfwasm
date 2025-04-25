(module $single-i32_eqz

    (func $i32_eqz (param i32) (result i32)
        (local.get 0) (i32.eqz)
    )

    (export "i32_eqz" (func $i32_eqz))
)
