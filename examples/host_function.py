from wasmer import Store, Module, Instance, ImportObject, Function
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

store = Store()
module = Module(store, open(__dir__ + '/imported_function.wasm', 'rb').read())

def sum(x: int, y: int) -> int:
    return x + y

import_object = ImportObject()
import_object.register(
    "env",
    {
        "sum": Function(store, sum),
    }
)

instance = Instance(module, import_object)

print(instance.exports.sum_plus_one(1, 2))
