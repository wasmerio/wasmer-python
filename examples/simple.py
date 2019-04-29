from wasmer import Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)

result = instance.exports.sum(1, 2)

print(result) # 3!
