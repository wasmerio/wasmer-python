from wasmer import Instance
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

# Instantiates the module.
wasm_bytes = open(__dir__ + '/greet.wasm', 'rb').read()
instance = Instance(wasm_bytes)

# Set the subject to greet.
subject = bytes('Wasmer 🐍', 'utf-8')
length_of_subject = len(subject)

# Allocate memory for the subject, and get a pointer to it.
input_pointer = instance.exports.allocate(length_of_subject)

# Write the subject into the memory.
memory = instance.memory.uint8_view(input_pointer)

for nth in range(0, length_of_subject):
    memory[nth] = subject[nth]

# C-string terminates by NULL.
memory[length_of_subject] = 0

# Run the `greet` function. Give the pointer to the subject.
output_pointer = instance.exports.greet(input_pointer)

# Read the result of the `greet` function.
memory = instance.memory.uint8_view(output_pointer)

output = []
nth = 0

while True:
    byte = memory[nth]

    if byte == 0:
        break

    output.append(byte)
    nth += 1

length_of_output = nth

print(bytes(output).decode())

# Deallocate the subject, and the output.
instance.exports.deallocate(input_pointer, length_of_subject)
instance.exports.deallocate(output_pointer, length_of_output)
