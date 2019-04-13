from wasmer import Instance, Value, Uint8MemoryView
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
print(instance.exports)
print(instance.exports['sum'](1, 4))
#result = instance.call('sum', [Value.i32(5), Value.i32(37)])

#print(result) # 42!
