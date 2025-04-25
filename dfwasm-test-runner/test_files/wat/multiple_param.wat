(module

    (func $divide (param $a i32) (param $b i32) (result i32)
        (i32.div_s (local.get $a) (local.get $b))
    )

    (func $multiple_param (export "multiple_param") (param $a i32) (param $b i32) (param $c i32) (result i32)
        (call $divide (local.get $a) (local.get $b))
        (call $divide (local.get $b) (local.get $c))
        i32.add
    )

)