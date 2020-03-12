from wasmer import Instance
import os
import gc

gc.disable()

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/imported_function.wasm', 'rb').read()

def sum(x: 'i32', y: 'i32') -> 'i32':
    return x + y

imports = {"env": {"sum": sum}}

instance = Instance(
    wasm_bytes,
    imports
)

print(instance.exports.add_one(1, 2))
