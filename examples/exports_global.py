# A Wasm module can export entities, like functions, memories, globals
# and tables.
#

# This example illustrates how to use exported globals. They come in 2
# flavors:
#
#   1. Immutable globals (const),
#   2. Mutable globals.
#
# You can run the example directly by executing in Wasmer root:#
#
# ```shell
# $ python examples/exports_global.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, Instance, Global, Type
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
wasm_bytes = wat2wasm(
    """
    (module
      (global $one (export "one") f32 (f32.const 1))
      (global $some (export "some") (mut f32) (f32.const 0))
      (func (export "get_one") (result f32) (global.get $one))
      (func (export "get_some") (result f32) (global.get $some))
      (func (export "set_some") (param f32) (global.set $some (local.get 0))))
    """
)

# Create a store.
store = Store(engine.JIT(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Let's instantiate the Wasm module.
instance = Instance(module)

# Here we go.
#
# The Wasm module exports some globals. Let's get them.
one = instance.exports.one
some = instance.exports.some

one_type = one.type
assert one_type.type == Type.F32
assert one_type.mutable == False

some_type = some.type
assert some_type.type == Type.F32
assert some_type.mutable == True

# Getting the values of globals can be done in two ways:
#
# 1. Through an exported function,
# 2. Using the Global API directly.
#
# We will use an exported function for the `one` global and the Global
# API for `some`.
get_one = instance.exports.get_one

one_value = get_one()
some_value = some.value

assert one_value == 1.0
assert one.value == 1.0
assert some_value == 0.0

# Trying to set the value of a immutable global (`const`) will result
# in an exception.
try:
    one.value = 42.0
except RuntimeError as err:
    assert str(err) == 'The global variable is not mutable, cannot set a new value'
else:
    assert False

# Setting the values of globals can be done in two ways:
#
# 1. Through an exported function,
# 2. Using the Global API directly.
#
# We will use both for the `some` global.
instance.exports.set_some(21.0)
assert some.value == 21.0

some.value = 42.0
assert some.value == 42.0
