(module $call
    (func $add (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.add
    )

    (func $call (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        call $add
    )

    (export "call" (func $call))
)