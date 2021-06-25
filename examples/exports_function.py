# A Wasm module can export entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use exported functions.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/exports_function.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, Instance, Function
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

# Create a store.
store = Store(engine.Universal(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Let's instantiate the Wasm module.
instance = Instance(module)

# Here we go.
#
# An `Instance` has an `exports` getter, which returns the same
# `Exports` object (per `Instance`). `Exports.__getattr__` is the only
# API to get an export. It will return either a `Function`, a
# `Memory`, a `Global` or a `Table`.
#
# Let's call the `sum` function with 1 and 2.
results = instance.exports.sum(1, 2)

# But this is not always ideal. Keep in mind that a `Function` object
# is created everytime you call `Exports.__getattr__`. Hence the
# following solution to store the function inside a variable.
sum = instance.exports.sum

assert isinstance(sum, Function)

results = sum(1, 2)

# Did you notice something? We didn't cast the Python values
# (arguments of `sum`) to WebAssembly values. It's done automatically!
#
# Same for the results. It's casted to Python values automatically.

assert results == 3

# How cool is that :-)?
