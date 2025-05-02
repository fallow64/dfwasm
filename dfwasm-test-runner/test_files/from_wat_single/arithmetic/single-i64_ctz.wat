(module $single-i64_ctz

    (func $i64_ctz (param i64) (result i64)
        (local.get 0) (i64.ctz)
    )

    (export "i64_ctz" (func $i64_ctz))
)
