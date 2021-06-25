# A Wasm module can import and export entities, like functions,
# memories, globals and tables. This example illustrates the basics
# of using these entities.
#
# In this example we'll be using a sample Wasm module which exports
# some entities and requires us to also import some of them.
#
# The goal here is to give you an idea of how to work with imports and
# exports. We won't go into the details of each entities, they'll be
# covered in more details in the other examples.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/imports_exports.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, ImportObject, Function, Global, Instance, Type, Value
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
# We are using the text representation of the module here
wasm_bytes = wat2wasm(
    """
    (module
      (func $host_function (import "" "host_function") (result i32))
      (global $host_global (import "env" "host_global") i32)

      (func $function (export "guest_function") (result i32) (global.get $global))
      (global $global (export "guest_global") i32 (i32.const 42))
      (table $table (export "guest_table") 1 1 funcref)
      (memory $memory (export "guest_memory") 1))
    """
)

# Create a store. Engines and compilers are explained in other
# examples.
store = Store(engine.Universal(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Let's write the Python function that is going to be imported,
# i.e. called by the WebAssembly module.
def host_function_implementation() -> int:
    return 42

host_function = Function(store, host_function_implementation)

# Let's then create a global that is going to be imported.
host_global = Global(store, Value.i32(42))

# Create an import object.
#
# Imports are stored in namespaces. We'll need to register each of the
# namespaces with a name and add the imported entities there.
#
# Note that the namespace can also have an empty name.
#
# Our module requires us to import:
#   * A function `host_function` in a namespace with an empty name;
#   * A global `host_global` in the `env` namespace.
#
# Let's do this!
import_object = ImportObject()
import_object.register(
    "",
    {
        "host_function": host_function,
    }
)
import_object.register(
    "env",
    {
        "host_global": host_global,
    }
)

# Let's instantiate the module!
instance = Instance(module, import_object)

# And finally, let's play.
#
# The Wasm module exports some entities:
#
# * A function: `guest_function`,
# * A global: `guest_global`,
# * A memory: `guest_memory`,
# * A table: `guest_table`,
#
# Let's get them.
function = instance.exports.guest_function
print(function.type)

global_ = instance.exports.guest_global
print(global_.type)

memory = instance.exports.guest_memory
print(memory.type)

table = instance.exports.guest_table
print(table.type)
