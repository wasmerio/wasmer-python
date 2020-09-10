# A Wasm module can export entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use exported memories.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/exports_memory.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module, Instance, Memory
from wasmer_compiler_cranelift import Compiler

# Let's declare the Wasm module with the text representation.
# If this module was written in Rust, it would have been:
#
# ```rs
# #[no_mangle]
# pub extern fn hello() -> *const u8 {
#     b"Hello, World!\0".as_ptr()
# }
# ```
wasm_bytes = wat2wasm(
    """
    (module
      (type $hello_t (func (result i32)))
      (func $hello (type $hello_t) (result i32)
          i32.const 42)
      (memory $memory 1)
      (export "hello" (func $hello))
      (export "mem" (memory $memory))
      (data (i32.const 42) "Hello, World!"))
    """
)

# Create a store.
store = Store(engine.JIT(Compiler))

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Let's instantiate the Wasm module.
instance = Instance(module)

# OK, here go. First, let's call `hello`. It returns a pointer to the
# string in memory.
pointer = instance.exports.hello()

# Since the pointer is a constant here, it's easy to assert its value.
assert pointer == 42

# Now let's read the string. It lives in memory. Usually the main
# memory is named `memory`, but the sake of not being simple, the
# memory is named `mem` in our case.
memory = instance.exports.mem

# See, it's a `Memory`!
assert isinstance(memory, Memory)

# Next, read it. We have multiple options here. Either use the custom
# views (`Uint8Array`, `Int8Array`, `Uint16Array` etc.), or use a more
# idiomatic Pythonic approach with Python buffers + `bytearray`.
reader = bytearray(memory.buffer)

# Go read. We know `Hello, World!` is 13 bytes long.
#
# Don't forget that we read bytes. We need to decode them!
returned_string = reader[pointer:pointer + 13].decode()

assert returned_string == 'Hello, World!'

# Yeah B-)!
