import wasmer
from wasmer import Store, Module, ExportType, ImportType, FunctionType, MemoryType, GlobalType, TableType, Type
from enum import IntEnum
import os
import pytest

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

#def test_name_some():
#    assert Module(Store(), '(module $moduleName)').name == 'moduleName'

#def test_name_none():
#    assert Module(Store(), '(module)').name == None

#def test_name_set():
#    module = Module(Store(), '(module)')
#    module.name = 'hello'
#    assert module.name == 'hello'

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
    assert isinstance(exports[0].type, FunctionType)
    assert exports[0].type.params == [Type.I32, Type.I64]
    assert exports[0].type.results == []

    assert exports[1].name == "global"
    assert isinstance(exports[1].type, GlobalType)
    assert exports[1].type.type == Type.I32
    assert exports[1].type.mutable == False

    assert exports[2].name == "table"
    assert isinstance(exports[2].type, TableType)
    assert exports[2].type.type == Type.FUNC_REF
    assert exports[2].type.minimum == 0
    assert exports[2].type.maximum == None

    assert exports[3].name == "memory"
    assert isinstance(exports[3].type, MemoryType)
    assert exports[3].type.minimum == 1
    assert exports[3].type.maximum == None
    assert exports[3].type.shared == False

#def test_imports():
#    imports = Module(
#        Store(),
#        """
#        (module
#        (import "ns" "function" (func))
#        (import "ns" "global" (global f32))
#        (import "ns" "table" (table 1 2 anyfunc))
#        (import "ns" "memory" (memory 3 4)))
#        """
#    ).imports
#
#    assert isinstance(imports[0], ImportType)
#
#    assert imports[0].module == "ns"
#    assert imports[0].name == "function"
#    assert isinstance(imports[0].type, FunctionType)
#    assert imports[0].type.params == []
#    assert imports[0].type.results == []
#
#    assert imports[1].module == "ns"
#    assert imports[1].name == "global"
#    assert isinstance(imports[1].type, GlobalType)
#    assert imports[1].type.type == Type.F32
#    assert imports[1].type.mutable == False
#
#    assert imports[2].module == "ns"
#    assert imports[2].name == "table"
#    assert isinstance(imports[2].type, TableType)
#    assert imports[2].type.type == Type.FUNC_REF
#    assert imports[2].type.minimum == 1
#    assert imports[2].type.maximum == 2
#
#    assert imports[3].module == "ns"
#    assert imports[3].name == "memory"
#    assert isinstance(imports[3].type, MemoryType)
#    assert imports[3].type.minimum == 3
#    assert imports[3].type.maximum == 4
#    assert imports[3].type.shared == False

#def test_custom_section():
#    module = Module(Store(), open(here + '/custom_sections.wasm', 'rb').read())
#    assert module.custom_sections('easter_egg') == [b'Wasmer']
#    assert module.custom_sections('hello') == [b'World!']
#    assert module.custom_sections('foo') == []

#def test_serialize():
#    assert type(Module(Store(), "(module)").serialize()) == bytes
#
#def test_deserialize():
#    store = Store()
#
#    serialized_module = Module(
#        store,
#        """
#        (module
#          (func (export "function") (param i32 i64)))
#        """
#    ).serialize()
#    module = Module.deserialize(store, serialized_module)
#    del serialized_module
#
#    exports = module.exports
#
#    assert len(module.exports) == 1
#    assert exports[0].name == "function"
#    assert isinstance(exports[0].type, FunctionType)
#    assert exports[0].type.params == [Type.I32, Type.I64]
#    assert exports[0].type.results == []
