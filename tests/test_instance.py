import wasmer
from wasmer import Instance, Module, Store, Exports, Function
import inspect
import os
import pytest
import sys

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def value_with_type(value):
    return (value, type(value))

def test_version():
    assert isinstance(wasmer.__version__, str)

def test_core_version():
    assert isinstance(wasmer.__core_version__, str)

def test_new():
    assert isinstance(Instance(Module(Store(), TEST_BYTES)), Instance)

def test_exports():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert isinstance(instance.exports, Exports)

def test_exports_len():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert len(instance.exports) == 13

def test_export_does_not_exist():
    with pytest.raises(LookupError) as context_manager:
        Instance(Module(Store(), TEST_BYTES)).exports.foo

    exception = context_manager.value
    assert str(exception) == 'Export `foo` does not exist.'

def test_export_function():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert isinstance(instance.exports.sum, Function)
    assert value_with_type(instance.exports.sum(1, 2)) == (3, int)

def test_basic_sum():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.sum(1, 2)) == (3, int)

def test_call_arity_0():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.arity_0()) == (42, int)

def test_call_i32_i32():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.i32_i32(7)) == (7, int)

def test_call_i64_i64():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.i64_i64(7)) == (7, int)

def test_call_f32_f32():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.f32_f32(7.)) == (7., float)

def test_call_f64_f64():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.f64_f64(7.)) == (7., float)

def test_call_i32_i64_f32_f64_f64():
    assert round(Instance(Module(Store(), TEST_BYTES)).exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6), 6) == (
        1 + 2 + 3.4 + 5.6
    )

def test_call_bool_casted_to_i32():
    assert value_with_type(Instance(Module(Store(), TEST_BYTES)).exports.bool_casted_to_i32()) == (1, int)

def test_call_string():
    assert Instance(Module(Store(), TEST_BYTES)).exports.string() == 1048576

def test_call_void():
    assert Instance(Module(Store(), TEST_BYTES)).exports.void() == None
#
#def test_memory_view():
#    assert isinstance(Instance(TEST_BYTES).memory.uint8_view(), Uint8Array)
#
#@pytest.mark.skipif(sys.version_info < (3, 6), reason='require Python 3.6+ to run')
#def test_getfullargspec():
#    instance = Instance(TEST_BYTES)
#    assert instance.exports.sum.getfullargspec == inspect.FullArgSpec(
#        args=['x0', 'x1'],
#        varargs=None,
#        varkw=None,
#        defaults=None,
#        kwonlyargs=None,
#        kwonlydefaults=None,
#        annotations={'x0': Type.I32, 'x1': Type.I32, 'return': Type.I32}
#    )
#
#def test_resolve_exported_function():
#    assert Instance(TEST_BYTES).resolve_exported_function(0) == "sum"
