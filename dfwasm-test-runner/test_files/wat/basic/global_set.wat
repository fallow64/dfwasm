(module $global_set
    (global $g (mut i32) (i32.const 5))

    (func $global_set (param $a i32) (result i32)
        (local $b i32)
        global.get $g
        local.set $b

        local.get $a
        global.set $g
        local.get $b
    )

    (export "global_set" (func $global_set))
)