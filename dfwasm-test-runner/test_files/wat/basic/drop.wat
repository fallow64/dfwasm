(module $drop
    (func $drop (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        drop
    )

    (export "drop" (func $drop))
)