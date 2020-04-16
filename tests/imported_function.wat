(module
  (import "env" "sum" (func $f (param i32 i32) (result i32)))
  (func $a (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $f
    i32.const 1
    i32.add)
  (export "sum_plus_one" (func $a)))