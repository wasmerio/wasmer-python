# Defining an engine in Wasmer is one of the fundamental steps.
#
# This example illustrates how to use `wasmer.engine.Dylib`, aka the
# Dylib engine. An engine applies roughly 2 steps:
#
#   1. It compiles the Wasm module bytes to executable code, through
#      the intervention of a compiler,
#   2. It stores the executable code somewhere.
#
# In the particular context of the Dylib engine, the executable
# code is stored in a native object, more precisely in a dynamic
# library.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/engine_dylib.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
wasm_bytes = wat2wasm(
    """
    (module
      (type $sum_t (func (param i32 i32) (result i32)))
      (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
        local.get $x
        local.get $y
        i32.add)
      (export "sum" (func $sum_f)))
    """
)

# Define the engine that will drive everything.
#
# In this case, the engine is `wasmer.engine.Dylib` which means that a
# native object is going to be generated.
engine = engine.Dylib(Compiler)

# Create a store, that holds the engine.
store = Store(engine)

# Here we go.
#
# Let's compile the Wasm module. It is at this step that the Wasm text
# is transformed into Wasm bytes (if necessary), and then compiled to
# executable code by the compiler, which is then stored in memory by
# the engine.
module = Module(store, wasm_bytes)

# Congrats, the Wasm module is compiled! Now let's execute it for the
# sake of having a complete example.
#
# Let's instantiate the Wasm module.
instance = Instance(module)

# The Wasm module exports a function called `sum`.
sum = instance.exports.sum
results = sum(1, 2)

print(results)
assert results == 3
