from wasmer import Store, Module, Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

module = Module(Store(), open(__dir__ + '/add_one.wasm', 'rb').read())
instance = Instance(module)

result = instance.exports.add_one(1)

print(result) # 2!
