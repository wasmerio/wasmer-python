import wasmer
from wasmer import Store, Module, ExportType, ImportType, FunctionType, MemoryType, GlobalType, TableType, Type
from enum import IntEnum
import inspect
import os
import pytest
import sys

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def test_validate():
    assert Module.validate(Store(), TEST_BYTES)

def test_validate_invalid():
    assert not Module.validate(Store(), INVALID_TEST_BYTES)

def test_compile_bytes():
    assert isinstance(Module(Store(), TEST_BYTES), Module)

def test_compile_wat():
    assert isinstance(Module(Store(), '(module)'), Module)

def test_failed_to_compile():
    with pytest.raises(RuntimeError) as context_manager:
        Module(Store(), INVALID_TEST_BYTES)

def test_name_some():
    assert Module(Store(), '(module $moduleName)').name == 'moduleName'

def test_name_none():
    assert Module(Store(), '(module)').name == None

def test_name_set():
    module = Module(Store(), '(module)')
    module.name = 'hello'
    assert module.name == 'hello'

#def test_instantiate():
#    assert Module(TEST_BYTES).instantiate().exports.sum(1, 2) == 3

def test_exports():
    exports = Module(
        Store(),
        """
        (module
          (func (export "function") (param i32 i64))
          (global (export "global") i32 (i32.const 7))
          (table (export "table") 0 funcref)
          (memory (export "memory") 1))
        """
    ).exports

    assert isinstance(exports[0], ExportType)

    assert exports[0].name == "function"
    assert isinstance(exports[0].ty, FunctionType)
    assert exports[0].ty.params == [Type.I32, Type.I64]
    assert exports[0].ty.results == []

    assert exports[1].name == "global"
    assert isinstance(exports[1].ty, GlobalType)
    assert exports[1].ty.ty == Type.I32
    assert exports[1].ty.mutable == False

    assert exports[2].name == "table"
    assert isinstance(exports[2].ty, TableType)
    assert exports[2].ty.ty == Type.FUNC_REF
    assert exports[2].ty.minimum == 0
    assert exports[2].ty.maximum == None

    assert exports[3].name == "memory"
    assert isinstance(exports[3].ty, MemoryType)
    assert exports[3].ty.minimum == 1
    assert exports[3].ty.maximum == None
    assert exports[3].ty.shared == False

def test_imports():
    imports = Module(
        Store(),
        """
        (module
          (import "ns" "function" (func $f))
          (import "ns" "global" (global $g f32))
          (import "ns" "table" (table $t 1 2 anyfunc))
          (import "ns" "memory" (memory $m 3 4)))
        """
    ).imports

    assert isinstance(imports[0], ImportType)

    assert imports[0].module == "ns"
    assert imports[0].name == "function"
    assert isinstance(imports[0].ty, FunctionType)
    assert imports[0].ty.params == []
    assert imports[0].ty.results == []

    assert imports[1].module == "ns"
    assert imports[1].name == "global"
    assert isinstance(imports[1].ty, GlobalType)
    assert imports[1].ty.ty == Type.F32
    assert imports[1].ty.mutable == False

    assert imports[2].module == "ns"
    assert imports[2].name == "table"
    assert isinstance(imports[2].ty, TableType)
    assert imports[2].ty.ty == Type.FUNC_REF
    assert imports[2].ty.minimum == 1
    assert imports[2].ty.maximum == 2

    assert imports[3].module == "ns"
    assert imports[3].name == "memory"
    assert isinstance(imports[3].ty, MemoryType)
    assert imports[3].ty.minimum == 3
    assert imports[3].ty.maximum == 4
    assert imports[3].ty.shared == False

def test_custom_section():
    module = Module(Store(), open(here + '/custom_sections.wasm', 'rb').read())
    assert module.custom_sections('easter_egg') == [b'Wasmer']
    assert module.custom_sections('hello') == [b'World!']
    assert module.custom_sections('foo') == []

def test_serialize():
    assert type(Module(Store(), "(module)").serialize()) == bytes

def test_deserialize():
    store = Store()

    serialized_module = Module(
        store,
        """
        (module
          (func (export "function") (param i32 i64)))
        """
    ).serialize()
    module = Module.deserialize(store, serialized_module)
    del serialized_module

    exports = module.exports

    assert len(module.exports) == 1
    assert exports[0].name == "function"
    assert isinstance(exports[0].ty, FunctionType)
    assert exports[0].ty.params == [Type.I32, Type.I64]
    assert exports[0].ty.results == []
