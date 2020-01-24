(module
  (import "ns" "f1" (func $f1))
  (import "ns" "f2" (func $f2))
  (import "ns" "m1" (memory $m1 3 4))
  (import "ns" "g1" (global $g1 f32))
  (import "ns" "t1" (table $t 1 2 anyfunc)))