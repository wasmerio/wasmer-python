from wasm import Instance, Value
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(bytes)
result = instance.call('sum', [Value.from_i32(5), Value.from_i32(37)])

print(result) # 42!
