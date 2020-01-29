(module
  (func $f1 (result i32) (i32.const 7))
  (func $f2 (result i32) (i32.const 42))
  (table (export "table1") anyfunc (elem $f1 $f2)))