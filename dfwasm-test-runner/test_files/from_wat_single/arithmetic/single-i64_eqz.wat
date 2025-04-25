(module $single-i64_eqz

    (func $i64_eqz (param i64) (result i32)
        (local.get 0) (i64.eqz)
    )

    (export "i64_eqz" (func $i64_eqz))
)
