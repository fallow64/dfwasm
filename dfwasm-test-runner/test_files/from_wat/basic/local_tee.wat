(module $local_tee
    (func $local_tee (param $a i32) (result i32)
        (local $x i32)

        local.get $a
        local.tee $x
    )

    (export "local_tee" (func $local_tee))
)