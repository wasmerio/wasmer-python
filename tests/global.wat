(module
 (global $x (export "x") (mut i32) (i32.const 0))
 (global $y (export "y") (mut i32) (i32.const 7))
 (global $z (export "z") i32 (i32.const 42))

 (func (export "get_x") (result i32)
   (global.get $x))

 (func (export "increment_x")
   (global.set $x
     (i32.add (global.get $x) (i32.const 1)))))