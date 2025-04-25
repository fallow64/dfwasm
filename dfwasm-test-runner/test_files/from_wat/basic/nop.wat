(module $nop
    (func $nop (param $a i32) (result i32)
        local.get $a
        nop
    )

    (export "nop" (func $nop))
)