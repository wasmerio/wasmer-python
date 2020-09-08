from wasmer import Store, Module, Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

module = Module(Store(), open(__dir__ + '/memory.wasm', 'rb').read())
instance = Instance(module)
pointer = instance.exports.return_hello()

memory = instance.exports.memory.uint8_view(pointer)
memory_length = len(memory)

nth = 0;
string = '';

while nth < memory_length:
    char = memory[nth]

    if char == 0:
        break;

    string += chr(char)
    nth += 1

print('"' + string + '"') # "Hello, World!"
print('"' + ''.join(map(chr, memory[0:13])) + '"')
