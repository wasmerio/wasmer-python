from wasmer import Instance, Value
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/memory.wasm', 'rb').read()
instance = Instance(wasm_bytes)
pointer = instance.call('return_hello')

memory = instance.uint8_memory_view(pointer)
nth = 0;
string = '';

while (0 != memory[nth]):
    string += chr(memory[nth])
    nth += 1

print('"' + string + '"') # "Hello, World!"
