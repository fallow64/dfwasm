(module $nested_branch
    (func $nested_branch (export "nested_branch") (param $n i32) (result i32)
        (local $i i32)
        (local.set $i (i32.const 0))
        (block $block1
            (loop $loop1
                ;; If $i >= $n, break
                (i32.ge_s (local.get $i) (local.get $n))
                (if
                    (then
                        (br $block1)
                    )
                )

                ;; Increment counter
                (local.set $i (i32.add (local.get $i) (i32.const 1)))

                ;; Back to start of loop
                (block
                    br $loop1
                )
            )
        )
        (local.get $i)
    )
)