from wasmer import Instance, Value, Uint8MemoryView
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
memory = instance.uint8_memory_view()
print(len(memory))
print(memory[42])
print(memory.BYTES_PER_ELEMENT)
print(Uint8MemoryView.BYTES_PER_ELEMENT)
#result = instance.call('sum', [Value.i32(5), Value.i32(37)])

#print(result) # 42!
