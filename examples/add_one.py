from wasmer import Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/add_one.wasm', 'rb').read()
instance = Instance(wasm_bytes)

result = instance.exports.add_one(1)

print(result) # 2!
