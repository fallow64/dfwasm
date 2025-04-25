(module
  (memory (export "memory") 1)

  (func $program_data (export "progam_data") (result i32)
    ;; Load the character at offset 0
    i32.const 0
    i32.load
    ;; Compare with ASCII for 'Z' (90)
    i32.const 90
    i32.ne
    if
        ;; If not 'Z', return 0
        i32.const 0
        return
    end

    ;; Load the character at offset 0
    i32.const 8
    i32.load
    ;; Compare with ASCII for ' ' (32)
    i32.const 32
    i32.ne
    if
        ;; If not ' ', return 0
        i32.const 0
        return
    end

    ;; Load the character at offset 24
    i32.const 8
    i32.load
    ;; Compare against 0 (the null terminator)
    i32.const 0
    i32.ne
    if
        ;; If not \00, return 0
        i32.const 0
        return
    end

    ;; Load the character at offset 1024+7
    i32.const 7
    i32.load offset=1024
    ;; Compare against ASCII for 'u' (117)
    i32.const 117
    i32.ne
    if
        ;; If not 117, return 0
        i32.const 0
        return
    end

    ;; Load the character at offset 4096+18
    i32.const 18
    i32.load offset=4096
    ;; Compare against ASCII for ',' (44)
    i32.const 44
    i32.ne
    if
        ;; If not 44, return 0
        i32.const 0
        return
    end

    ;; Load the character at offset 4096+19
    i32.const 19
    i32.load offset=4096
    ;; Compare against ASCII for ' ' (44)
    i32.const 32
    i32.ne
    if
        ;; If not ' ', return 0
        i32.const 0
        return
    end

    ;; all good!
    i32.const 1
  )

  (data $.rodata (i32.const 0) "Ziltoid, the Omniscient!\00")
  (data $.data (i32.const 1024) "in search of the ultimate cup of coffee\00")
  (data $.data.1 (i32.const 2048) "you have five Earth minutes... make it perfect!\00")

  ;; wtf is this syntax lmao
  (data $.data.2 (i32.const 4096) "I am so omniscient," " if there were to be two omnisciences.." " I would be both!\00")
)