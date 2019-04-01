import wasm

a = wasm.Instance()
print(a.invoke_function("add_one"))
