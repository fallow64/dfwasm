(module $single-i64_sub

    (func $i64_sub (param i64 i64) (result i64)
        (local.get 0) (local.get 1) (i64.sub)
    )

    (export "i64_sub" (func $i64_sub))
)
