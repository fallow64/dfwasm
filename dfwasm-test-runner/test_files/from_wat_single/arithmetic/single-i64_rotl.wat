(module $single-i64_rotl

    (func $i64_rotl (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.rotl)
    )

    (export "i64_rotl" (func $i64_rotl))
)
