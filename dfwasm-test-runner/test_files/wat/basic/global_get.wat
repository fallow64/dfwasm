(module $global_get
    (global $g (mut i32) (i32.const 5))

    (func $get_global (result i32)
        global.get $g
    )
    
    (export "get_global" (func $get_global))
)