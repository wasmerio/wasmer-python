import wasm
import os

here = os.path.dirname(os.path.realpath(__file__))

file = open(here + "/simple.wasm", "rb");
bytecode = file.read()

instance = wasm.Instance(bytecode)
function_name = "sum"
function_arguments = [wasm.Value.from_i32(1), wasm.Value.from_i32(3)]

print(instance.call(function_name, function_arguments))
