# A Wasm module can be compiled with multiple compilers.
#
# This example illustrates how to use the Singlepass compiler.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/compiler_singlepass.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, Instance
from wasmer_compiler_singlepass import Compiler

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
# In this case, the engine is `wasmer.engine.Universal` which roughly
# means that the executable code will live in memory.
#
# This is _the_ place to pass the compiler. Note that `Compiler` is
# not instantiated, we pass the class only.
engine = engine.Universal(Compiler)

# Create a store, that holds the engine.
store = Store(engine)

# Let's compile the Wasm module with the Singlepass compiler.
module = Module(store, wasm_bytes)

# Let's instantiate the Wasm module.
instance = Instance(module)

# Let's call the `sum` exported function.
sum = instance.exports.sum
results = sum(1, 2)

print(results)
assert results == 3
