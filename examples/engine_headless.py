# Defining an engine in Wasmer is one of the fundamental steps.
#
# This example illustrates a neat feature of engines: their ability
# to run in a headless mode. At the time of writing, all engines
# have a headless mode, but it's not a requirement.
#
# What problem does it solve, and what does it mean?
#
# Once a Wasm module is compiled into executable code and stored
# somewhere (e.g. in memory with the JIT engine, or in a native
# object with the native engine), the module can be instantiated and
# executed. But imagine for a second the following scenario:
#
#   * Modules are compiled ahead of time, to be instantiated later
#     on.
#   * Modules are cross-compiled on a machine ahead of time
#     to be run on another machine later one.
#
# In both scenarios, the environment where the compiled Wasm module
# will be executed can be very constrained. For such particular
# contexts, Wasmer can be compiled _without_ the compilers, so that
# the `wasmer` package is as small as possible. Indeed, there is no
# need for a compiler since the Wasm module is already compiled. All
# we need is an engine that _only_ drives the instantiation and
# execution of the Wasm module.
#
# And that, that's a headless engine.
#
# To achieve such a scenario, a Wasm module must be compiled, then
# serialized —for example into a file—, then later, potentially on
# another machine, deserialized. The next steps are classical: The
# Wasm module is instantiated and executed.
#
# This example uses a compiler because it illustrates the entire
# workflow, but keep in mind the compiler isn't required after the
# compilation step.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/engine_headless.py
# ```
#
# Ready?

from wasmer import engine, wat2wasm, Store, Module

# First step, let's compile the Wasm module and serialize it.
# Note: we need a compiler here.

from wasmer_compiler_cranelift import Compiler
import tempfile

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
# In this case, the engine is `wasmer.engine.Native` which means that
# a native object is going to be generated. So when we are going to
# serialize the compiled Wasm module, we are going to store it in a
# file with the `.so` extension for example (or `.dylib`, or `.dll`
# depending of the platform).
engine = engine.Native(Compiler)

# Create a store, that holds the engine.
store = Store(engine)

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

# Here we go. Let's serialize the compiled Wasm module in a file.
serialized_module = module.serialize()

serialized_module_file = tempfile.TemporaryFile()
serialized_module_file.write(serialized_module)

# We seek to the initial position, so when we read it starts from the beginning
serialized_module_file.seek(0, 0)


# Second step, deserialize the compiled Wasm module, and execute it,
# for example with Wasmer without a compiler.

from wasmer import engine, Store, Instance, Module

# We create a headless native engine, i.e. an engine without a
# compiler.
engine = engine.Native()

# Create a store, as usual.
store = Store(engine)

# Here we go.
#
# Deserialize the compiled Wasm module. This code is unsafe because
# Wasmer can't assert the bytes are valid, so be careful.
module = Module.deserialize(store, serialized_module_file.read())

# Congrats, the Wasm module has been deserialized! Now let's execute
# it for the sake of having a complete example.

# Let's instantiate the Wasm module.
instance = Instance(module)

# The Wasm module exports a function called `sum`.
results = instance.exports.sum(1, 2)

assert results == 3
print(results)
