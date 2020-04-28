from wasmer import Instance, Module, ImportObject
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/imported_function.wasm', 'rb').read()

def sum(x: int, y: int) -> int:
    return x + y

instance = Instance(
    wasm_bytes,
    {
        "env": {
            "sum": sum
        }
    }
)

print(instance.exports.sum_plus_one(1, 2))

module = Module(wasm_bytes)
import_object = module.generate_import_object()
import_object.extend({
    "env": {
        "sum": sum
    }
})
instance = module.instantiate(import_object)

print(instance.exports.sum_plus_one(3, 4))
