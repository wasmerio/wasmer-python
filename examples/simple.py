from wasmer import Instance, Value, Uint8MemoryView
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
result = instance.exports['sum'](5, 37)

print(result) # 42!
