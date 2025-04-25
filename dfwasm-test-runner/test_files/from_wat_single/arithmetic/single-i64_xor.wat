(module $single-i64_xor

    (func $i64_xor (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.xor)
    )

    (export "i64_xor" (func $i64_xor))
)
