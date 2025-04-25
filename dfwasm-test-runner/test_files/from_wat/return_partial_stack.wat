(module
  (func $f0
    ;; stack: []
    i32.const 20
    i32.const 5
    ;; stack: [20, 5]
    return
    ;; stack: [5]
  )

  (func $return_partial_stack (export "return_partial_stack") (result i32)
    ;; stack: []
    (i32.const 10)
    ;; stack: [10]
    (call $f0)
    ;; stack: [10, 5]
    return
    ;; stack: [5]
  )
)