from wasmer import Store, Module, Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

module = Module(Store(), open(__dir__ + '/simple.wasm', 'rb').read())
instance = Instance(module)

result = instance.exports.sum(1, 2)

print(result) # 3!
