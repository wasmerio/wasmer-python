# Running a WASI compiled WebAssembly module with Wasmer.
#
# This example illustrates how to run WASI modules with
# Wasmer. To run WASI we have to have to do mainly 3 steps:
#
#   1. Create a `wasi.Environment` instance,
#   2. Attach the imports from the `wasi.Environment` to a new
#      instance,
#   3. Run the WASI module.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/wasi.py
# ```
#
# Ready?

from wasmer import engine, wasi, Store, Module, ImportObject, Instance
from wasmer_compiler_cranelift import Compiler
import os

# Let's get the `wasi.wasm` bytes!
__dir__ = os.path.dirname(os.path.realpath(__file__))
wasm_bytes = open(__dir__ + '/appendices/wasi.wasm', 'rb').read()

# Create a store.
store = Store(engine.JIT(Compiler))

# Let's compile the Wasm module, as usual.
module = Module(store, wasm_bytes)

# Here we go.
#
# First, let's extract the WASI version from the module. Why? Because
# WASI already exists in multiple versions, and it doesn't work the
# same way. So, to ensure compatibility, we need to know the version.
wasi_version = wasi.get_version(module, strict=True)

# Second, create a `wasi.Environment`. It contains everything related
# to WASI. To build such an environment, we must use the
# `wasi.StateBuilder`.
#
# In this case, we specify the program name is `wasi_test_program`. We
# also specify the program is invoked with the `--test` argument, in
# addition to two environment variable: `COLOR` and
# `APP_SHOULD_LOG`. Finally, we map the `the_host_current_dir` to the
# current directory. There it is:
wasi_env = \
    wasi.StateBuilder('wasi_test_program'). \
        argument('--test'). \
        environment('COLOR', 'true'). \
        environment('APP_SHOULD_LOG', 'false'). \
        map_directory('the_host_current_dir', '.'). \
        finalize()

# From the WASI environment, we generate a custom import object. Why?
# Because WASI is, from the user perspective, a bunch of
# imports. Consequently `generate_import_object`… generates a
# pre-configured import object.
#
# Do you remember when we said WASI has multiple versions? Well, we
# need the WASI version here!
import_object = wasi_env.generate_import_object(store, wasi_version)

# Now we can instantiate the module.
instance = Instance(module, import_object)

# One last thing. This WASI module expects a memory! But it has no
# memory “instance”. So let's bind the WASI memory and the instance
# exported memory together.
wasi_env.memory = instance.exports.memory

# The entry point for a WASI WebAssembly module is a function named
# `_start`. Let's call it and see what happens!
instance.exports._start()

# It has printed:
#
# ```
# Found program name: `wasi_test_program`
# Found 1 arguments: --test
# Found 2 environment variables: COLOR=true, APP_SHOULD_LOG=false
# Found 1 preopened directories: DirEntry("/the_host_current_dir")
# ```
#
# on the standard output.
