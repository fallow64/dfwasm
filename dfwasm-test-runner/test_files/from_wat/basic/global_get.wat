(module $global_get
    (global $g (mut i32) (i32.const 5))

    (func $global_get (result i32)
        global.get $g
    )
    
    (export "global_get" (func $global_get))
)