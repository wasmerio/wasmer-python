import wasmer
from wasmer import Instance, Uint8Array, Value, Type
import inspect
import os
import pytest
import sys

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def value_with_type(value):
    return (value, type(value))

def test_is_a_class():
    assert inspect.isclass(Instance)

def test_version():
    assert isinstance(wasmer.__version__, str)

def test_core_version():
    assert isinstance(wasmer.__core_version__, str)

def test_can_construct():
    assert isinstance(Instance(TEST_BYTES), Instance)

def test_failed_to_instantiate():
    with pytest.raises(RuntimeError) as context_manager:
        Instance(INVALID_TEST_BYTES)

    exception = context_manager.value
    assert str(exception) == (
        'Failed to compile the module:\n    Validation error "Invalid type"'
    )

def test_function_does_not_exist():
    with pytest.raises(LookupError) as context_manager:
        Instance(TEST_BYTES).exports.foo

    exception = context_manager.value
    assert str(exception) == 'Function `foo` does not exist.'

def test_basic_sum():
    assert value_with_type(Instance(TEST_BYTES).exports.sum(1, 2)) == (3, int)

def test_call_arity_0():
    assert value_with_type(Instance(TEST_BYTES).exports.arity_0()) == (42, int)

def test_call_i32_i32():
    assert value_with_type(Instance(TEST_BYTES).exports.i32_i32(7)) == (7, int)

def test_call_i64_i64():
    assert value_with_type(Instance(TEST_BYTES).exports.i64_i64(7)) == (7, int)

def test_call_f32_f32():
    assert value_with_type(Instance(TEST_BYTES).exports.f32_f32(7.)) == (7., float)

def test_call_f64_f64():
    assert value_with_type(Instance(TEST_BYTES).exports.f64_f64(7.)) == (7., float)

def test_call_i32_i64_f32_f64_f64():
    assert round(Instance(TEST_BYTES).exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6), 6) == (
        1 + 2 + 3.4 + 5.6
    )

def test_call_bool_casted_to_i32():
    assert value_with_type(Instance(TEST_BYTES).exports.bool_casted_to_i32()) == (1, int)

def test_call_string():
    assert Instance(TEST_BYTES).exports.string() == 1048576

def test_call_void():
    assert Instance(TEST_BYTES).exports.void() == None

def test_memory_view():
    assert isinstance(Instance(TEST_BYTES).memory.uint8_view(), Uint8Array)

@pytest.mark.skipif(sys.version_info < (3, 6), reason='require Python 3.6+ to run')
def test_getfullargspec():
    instance = Instance(TEST_BYTES)
    assert instance.exports.sum.getfullargspec == inspect.FullArgSpec(
        args=['x0', 'x1'],
        varargs=None,
        varkw=None,
        defaults=None,
        kwonlyargs=None,
        kwonlydefaults=None,
        annotations={'x0': Type.I32, 'x1': Type.I32, 'return': Type.I32}
    )

def test_resolve_exported_function():
    assert Instance(TEST_BYTES).resolve_exported_function(0) == "sum"
