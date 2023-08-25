(module
  (import "env" "import" (func $imp))
  (func $init
    (i64.add (global.get $gas) (i64.const 1))
    global.set $gas
    call $imp
    call $init
  )
  (global $gas (mut i64) (i64.const 0))
  (export "init" (func $init))
  (export "gas" (global $gas))
)
