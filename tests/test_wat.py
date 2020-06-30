from wasmer import wat2wasm, wasm2wat, Instance

def test_wat2wasm():
    assert wat2wasm('(module)') == b'\x00asm\x01\x00\x00\x00'

def test_wasm2wat():
    assert wasm2wat(b'\x00asm\x01\x00\x00\x00') == '(module)'

def test_wat2wasm2instance():
    wat = """ (module
                (type (func (param i32 i32) (result i32)))
                (func (type 0)
                  local.get 0
                  local.get 1
                  i32.add)
                (export "sum" (func 0))) """
    wasm_bytes = wat2wasm(wat)
    instance = Instance(wasm_bytes)

    assert instance.exports.sum(1, 2) == 3
