(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (result i64)))
  (import "env" "memory" (memory (;0;) 261))
  (import "env" "gr_reply_push_input" (func (;0;) (type 5)))
  (import "env" "gr_debug" (func (;1;) (type 2)))
  (import "env" "gr_out_of_gas" (func (;2;) (type 0)))
  (func (;3;) (type 0)
    (local i64)
    i32.const 15820
    call 7
    global.get 0
    i32.const 28286
    i32.const 16845
    i32.const 13746590
    call 0
    i32.const 192
    call 7
    drop
  )
  (func (;4;) (type 6)
    loop  ;; label = @1
      i32.const 1487
      call 7
      i32.const 4424
      call 7
      call 5
      i32.const 3138
      call 7
      br 0 (;@1;)
    end
    unreachable
  )
  (func (;5;) (type 0)
    block  ;; label = @1
      i32.const 28571
      call 7
      call 3
      call 5
      br 0 (;@1;)
    end
  )
  (func (;6;) (type 0)
    i32.const 9232
    call 7
    i32.const 16777216
    i32.const 41
    call 1
    i32.const 12541
    call 7
    call 4
    drop
    unreachable
  )
  (func (;7;) (type 1)
    (local i64)
    local.get 0
    i64.extend_i32_u
    i64.const 26076
    i64.add
    local.tee 1
    global.get 2
    i64.gt_u
    if  ;; label = @1
      call 2
    end
    global.get 2
    local.get 1
    i64.sub
    global.set 2
  )
  (global (;0;) (mut i64) (i64.const -6165552189891260671))
  (global (;1;) i32 (i32.const 16777216))
  (global (;2;) (mut i64) (i64.const 0))
  (export "__gear_stack_end" (global 1))
  (export "init" (func 6))
  (export "gear_gas" (global 2))
  (data (;0;) (i32.const 16777216) "Gear program seed = '3161054873836996002'")
)
