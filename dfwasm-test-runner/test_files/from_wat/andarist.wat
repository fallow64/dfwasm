(module

  ;; ```c
  ;; int andarist(int x) {
  ;;     int a = x + 10;
  ;;     if (a > 5) {
  ;;         a = a + 2;
  ;;     }
  ;;     a = a + 7;
  ;;     return a;
  ;; }
  ;; ```

  (func $andarist (param $x i32) (result i32)
    (i32.add
      (i32.add
        (local.get $x)
        (select
          (i32.const 12)
          (i32.const 10)
          (i32.gt_s
            (local.get $x)
            (i32.const -5)
          )
        )
      )
      (i32.const 7)
    )
  )

  (export "andarist" (func $andarist))
)
