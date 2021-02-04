# A Wasm module can import entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use an imported function that fails!
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/imports_function_early_exit.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, ImportObject, Function, FunctionType, Type, Instance
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
wasm_bytes = wat2wasm(
    """
    (module
      (type $run_t (func (param i32 i32) (result i32)))
      (type $early_exit_t (func (param) (result)))

      (import "env" "early_exit" (func $early_exit (type $early_exit_t)))

      (func $run (type $run_t) (param $x i32) (param $y i32) (result i32)
        (call $early_exit)
        (i32.add
            local.get $x
            local.get $y))

      (export "run" (func $run)))
    """
)

# Create a store.
store = Store(engine.JIT(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Here we go.
#
# Let's write the Python function that is going to… fail!
def early_exit():
    raise Exception('oops')

# When creating an `Instance`, we can pass an `ImportObject`. All
# entities that must be imported are registered inside the
# `ImportObject`.
import_object = ImportObject()

# Now let's register the `sum` import inside the `env` namespace.
import_object.register(
    "env",
    {
        "early_exit": Function(store, early_exit),
    }
)

# Let's instantiate the module!
instance = Instance(module, import_object)

# And finally, call the `run` exported function!
try:
    instance.exports.run(1, 2)
except RuntimeError as err:
    assert 'oops' in str(err)
else:
    assert False
