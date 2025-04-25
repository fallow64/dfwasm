(module $call_indirect
    (type $f0 (func (param i32 i32) (result i32)))

    (table $t0 4 funcref)
    (elem $t0 (i32.const 0) func $add $sub $mul $div)

    (func $add (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.add
    )

    (func $sub (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.sub
    )

    (func $mul (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.mul
    )

    (func $div (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.div_s
    )

    (func $call_indirect (param $funcIndex i32) (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b

        (i32.rem_u (local.get $funcIndex) (i32.const 4))
        (call_indirect (type $f0))

        ;; local.get $a
        ;; local.get $b
        
        ;; (i32.rem_u (i32.add (local.get $funcIndex) (i32.const 1)) (i32.const 4))
        ;; (call_indirect (type $f0))

        ;; i32.add
    )

    (export "call_indirect" (func $call_indirect))
)