# Defining an engine in Wasmer is one of the fundamental steps.
#
# As a reminder, an engine applies roughly 2 steps:
#
#   1. It compiles the Wasm module bytes to executable code, through
#      the intervention of a compiler,
#   2. It stores the executable code somewhere.
#
# This example focuses on the first step: the compiler. It
# illustrates how the abstraction over the compiler is so powerful
# that it is possible to cross-compile a Wasm module.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ python examples/engine_cross_compilation.py
# ```
#
# Ready?

from wasmer import engine, target, wat2wasm, Store, Module
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

# Here we go.
#
# Let's define the target “triple”. Historically, such things had
# three fields, though additional fields have been added over time.
triple = target.Triple('x86_64-linux-musl')

# Here we go again.
#
# Let's define a CPU feature.
cpu_features = target.CpuFeatures()
cpu_features.add('sse2')

# Here we go finally.
#
# Let's build the target.
target = target.Target(triple, cpu_features)

# Define the engine that will drive everything.
#
# In this case, the engine is `wasmer.engine.Native` which means that
# a native object is going to be generated.
#
# That's where we specify the target for the compiler.
# Use the native engine.
engine = engine.Native(Compiler, target)

# Create a store, that holds the engine.
store = Store(engine)

# Let's compile the Wasm module.
module = Module(store, wasm_bytes)

assert isinstance(module, Module)

# Congrats, the Wasm module is cross-compiled!
#
# What to do with that? It is possible to use an engine (probably a
# headless engine) to execute the cross-compiled Wasm module an the
# targeted platform.
