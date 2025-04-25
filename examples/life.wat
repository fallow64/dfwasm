(module
  (type (;0;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 5
    i32.const 0
    i32.store
    local.get 4
    i32.const -1
    i32.add
    local.set 6
    local.get 4
    local.get 3
    i32.const 9
    i32.mul
    i32.add
    local.get 0
    i32.add
    i32.const -10
    i32.add
    local.set 7
    i32.const 0
    local.set 8
    i32.const -1
    local.set 0
    loop  ;; label = @1
      local.get 0
      local.get 3
      i32.add
      local.tee 9
      local.get 1
      i32.ge_s
      local.set 10
      i32.const 0
      local.set 4
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const -1
          i32.add
          local.get 0
          i32.or
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          local.get 10
          br_if 0 (;@3;)
          local.get 6
          local.get 4
          i32.add
          local.tee 11
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          local.get 11
          local.get 2
          i32.ge_s
          br_if 0 (;@3;)
          local.get 7
          local.get 4
          i32.add
          i32.load8_u
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 8
          i32.const 1
          i32.add
          local.tee 8
          i32.store
        end
        local.get 4
        i32.const 1
        i32.add
        local.tee 4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 7
      i32.const 9
      i32.add
      local.set 7
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.const 2
      i32.ne
      br_if 0 (;@1;)
    end)
  (func (;1;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      local.get 3
      i32.const 16
      i32.add
      local.set 5
      local.get 2
      i32.const 1
      i32.lt_s
      local.set 6
      local.get 0
      local.set 7
      loop  ;; label = @2
        block  ;; label = @3
          local.get 6
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          loop  ;; label = @4
            local.get 0
            local.get 1
            local.get 2
            local.get 4
            local.get 8
            local.get 3
            i32.const 12
            i32.add
            call 0
            local.get 5
            local.get 8
            i32.add
            local.get 3
            i32.load offset=12
            local.tee 9
            i32.const -2
            i32.and
            i32.const 2
            i32.eq
            local.get 9
            i32.const 3
            i32.eq
            local.get 7
            local.get 8
            i32.add
            i32.load8_u
            select
            i32.store8
            local.get 2
            local.get 8
            i32.const 1
            i32.add
            local.tee 8
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.const 9
        i32.add
        local.set 5
        local.get 7
        i32.const 9
        i32.add
        local.set 7
        local.get 4
        i32.const 1
        i32.add
        local.tee 4
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 0
      local.set 7
      local.get 3
      i32.const 16
      i32.add
      local.set 5
      local.get 2
      i32.const 1
      i32.lt_s
      local.set 6
      loop  ;; label = @2
        local.get 5
        local.set 8
        local.get 0
        local.set 9
        local.get 2
        local.set 4
        block  ;; label = @3
          local.get 6
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 9
            local.get 8
            i32.load8_u
            i32.store8
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 9
            i32.const 1
            i32.add
            local.set 9
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.const 9
        i32.add
        local.set 5
        local.get 0
        i32.const 9
        i32.add
        local.set 0
        local.get 7
        i32.const 1
        i32.add
        local.tee 7
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 112
    i32.add
    global.set 0)
  (func (;2;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      local.get 2
      i32.const 1
      i32.lt_s
      local.set 5
      i32.const 0
      local.set 3
      loop  ;; label = @2
        local.get 0
        local.set 6
        local.get 2
        local.set 7
        block  ;; label = @3
          local.get 5
          br_if 0 (;@3;)
          loop  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 78
              i32.gt_s
              br_if 0 (;@5;)
              local.get 3
              i32.const 1048576
              i32.add
              i32.const 36
              i32.const 46
              local.get 6
              i32.load8_u
              select
              i32.store8
              local.get 3
              i32.const 1
              i32.add
              local.set 3
            end
            local.get 6
            i32.const 1
            i32.add
            local.set 6
            local.get 7
            i32.const -1
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 3
          i32.const 79
          i32.ge_s
          br_if 0 (;@3;)
          local.get 3
          i32.const 1048576
          i32.add
          i32.const 10
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.set 3
        end
        local.get 0
        i32.const 9
        i32.add
        local.set 0
        local.get 4
        i32.const 1
        i32.add
        local.tee 4
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 1048576
    i32.add
    i32.const 0
    i32.store8
    i32.const 1048576)
  (func (;3;) (type 3) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 192
    i32.sub
    local.tee 1
    global.set 0
    i32.const 0
    local.set 2
    local.get 1
    i32.const 0
    i32.const 81
    memory.fill
    local.get 1
    i32.const 257
    i32.store16 offset=49 align=1
    local.get 1
    i32.const 257
    i32.store16 offset=39 align=1
    local.get 1
    i32.const 1
    i32.store8 offset=31
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 1
      i32.const 96
      i32.add
      local.get 2
      i32.add
      local.set 4
      local.get 1
      local.get 2
      i32.add
      local.set 5
      i32.const 0
      local.set 6
      loop  ;; label = @2
        local.get 4
        local.get 6
        i32.add
        local.get 5
        local.get 6
        i32.add
        i32.load8_u
        i32.store8
        local.get 6
        i32.const 1
        i32.add
        local.tee 6
        i32.const 9
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 9
      i32.add
      local.set 2
      local.get 3
      i32.const 1
      i32.add
      local.tee 3
      i32.const 9
      i32.ne
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 1
        i32.const 96
        i32.add
        i32.const 9
        i32.const 9
        call 1
        local.get 0
        i32.const -1
        i32.add
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 96
    i32.add
    i32.const 9
    i32.const 9
    call 2
    drop
    local.get 1
    i32.const 192
    i32.add
    global.set 0
    i32.const 1048576)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "run_and_display" (func 3)))
