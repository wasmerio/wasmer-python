# A Wasm module can import entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use imported functions, aka host
# functions.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/imports_function.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, ImportObject, Function, FunctionType, Type, Instance
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
# If this module was written in Rust, it would have been:
#
# ```rs
# extern "C" {
#     fn sum(x: i32, y: i32) -> i32;
# }
# 
# #[no_mangle]
# pub extern "C" fn add_one(x: i32) -> i32 {
#     unsafe { sum(x, 1) }
# }
# ```
wasm_bytes = wat2wasm(
    """
    (module
      (import "env" "sum" (func $sum (param i32 i32) (result i32)))
      (func (export "add_one") (param $x i32) (result i32)
        local.get $x
        i32.const 1
        call $sum))
    """
)

# Create a store.
store = Store(engine.Universal(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Here we go.
#
# When creating an `Instance`, we can pass an `ImportObject`. All
# entities that must be imported are registered inside the
# `ImportObject`.
import_object = ImportObject()

# Let's write the Python function that is going to be imported,
# i.e. called by the WebAssembly module.
def sum(x: int, y: int) -> int:
    return x + y

sum_host_function = Function(store, sum)

# See how we have used Python annotations to help `wasmer` to infer
# the types of the host function? Well, it could be limited. For
# example, `int` in Python matches to `i32` in WebAssembly. We can't
# represent `i64`. Or… we can use a function type.
def sum(x, y):
    return x + y

sum_host_function = Function(
    store,
    sum,
    #             x         y           result
    FunctionType([Type.I32, Type.I32], [Type.I32])
)

# Now let's register the `sum` import inside the `env` namespace.
import_object.register(
    "env",
    {
        "sum": sum_host_function,
    }
)

# Let's instantiate the module!
instance = Instance(module, import_object)

# And finally, call the `add_one` exported function!
assert instance.exports.add_one(41) == 42
