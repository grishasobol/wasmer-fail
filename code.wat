(module
  (import "env" "import" (func $imp (param i32 i32 i32)))
  (func $init
    (i64.add (global.get $gas) (i64.const 1))
    global.set $gas
    i32.const 1
    i32.const 2
    i32.const 3
    call $imp
    call $init
  )
  (global $gas (mut i64) (i64.const 0))
  (export "init" (func $init))
  (export "gas" (global $gas))
)
