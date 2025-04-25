;; to https://musteresel.github.io/posts/2020/01/webassembly-text-br_table-example.html .. thanks bro. couldn't find them either

(module
  (func $br_table (param $p i32) (result i32)
    ;; switch-like
    (block $B0
      (block $B1
        (block $B2
          (block $B3
            (local.get $p)
            (br_table
              $B2   ;; if $p == 0 then branch to $B2
              $B1   ;; if $p == 1 then branch to $B1
              $B0   ;; if $p == 2 then branch to $B0
              $B3   ;; else => $B3
            )
          )
          
          ;; Target for $B0
          (i32.const 100)
          (return)
        )

        ;; Target for $B1
        (i32.const 101)
        (return)
      )

      ;; Target for $B2
      (i32.const 102)
      (return)
    )

    ;; Target for $B3
    (i32.const 103)
    (return)
  )

  (export "br_table" (func $br_table))
)