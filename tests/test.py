import wasm

instance = wasm.Instance()
function_name = "add_one"
function_arguments = [wasm.Value.from_i32(2)]

print(instance.call(function_name, function_arguments))
