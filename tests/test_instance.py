import wasmer
from wasmer import Instance, Module, Store, Exports, Function, Global, Table, Memory
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def test_version():
    assert isinstance(wasmer.__version__, str)

def test_core_version():
    assert isinstance(wasmer.__core_version__, str)

def test_new():
    assert isinstance(Instance(Module(Store(), TEST_BYTES)), Instance)

def test_exports():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert isinstance(instance.exports, Exports)

def test_exports_all_kind():
    module = Module(
        Store(),
        """
        (module
          (func (export "func") (param i32 i64))
          (global (export "glob") i32 (i32.const 7))
          (table (export "tab") 0 funcref)
          (memory (export "mem") 1))
        """
    )
    instance = Instance(module)
    exports = instance.exports

    assert isinstance(exports, Exports)
    assert isinstance(exports.func, Function)
    assert isinstance(exports.glob, Global)
    assert isinstance(exports.tab, Table)
    assert isinstance(exports.mem, Memory)

def test_exports_not_clone():
    instance = Instance(Module(Store(), TEST_BYTES))
    exports1 = instance.exports
    exports2 = instance.exports

    assert exports1 == exports2

def test_exports_len():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert len(instance.exports) == 13

def test_export_does_not_exist():
    with pytest.raises(LookupError) as context_manager:
        Instance(Module(Store(), TEST_BYTES)).exports.foo

    exception = context_manager.value
    assert str(exception) == 'Export `foo` does not exist.'
