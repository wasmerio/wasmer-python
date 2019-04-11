from wasmer import Instance, Value
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
result = instance.call('sum', [Value.i32(5), Value.i32(37)])

print(result) # 42!
