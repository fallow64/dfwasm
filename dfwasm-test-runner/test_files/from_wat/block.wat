(module $block
    (func $block (export "block") (param $n i32) (result i32)
        (local.set $n (i32.add (local.get $n) (i32.const 1)))
        (block $block1
            (local.set $n (i32.add (local.get $n) (i32.const 1)))
            br $block1
            (local.set $n (i32.add (local.get $n) (i32.const 1)))
        )
        (local.set $n (i32.add (local.get $n) (i32.const 1)))
        (local.get $n)
    )
)