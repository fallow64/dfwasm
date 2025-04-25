(module
  (func $select (export "select") (param $condition i32) (result i32)
    (select
      (i32.const 10) ;; first operand
      (i32.const 20) ;; second operand
      (local.get $condition) ;; condition (if equal to zero the second operand will be selected, otherwise the first will be selected)
    )
  )
)