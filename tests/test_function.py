import wasmer
from wasmer import Instance, Module, Store, Function
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def instance():
    return Instance(Module(Store(), TEST_BYTES))

def value_with_type(value):
    return (value, type(value))

def test_export_function():
    assert isinstance(instance().exports.sum, Function)

def test_basic_sum():
    assert value_with_type(instance().exports.sum(1, 2)) == (3, int)

def test_call_arity_0():
    assert value_with_type(instance().exports.arity_0()) == (42, int)

def test_call_i32_i32():
    assert value_with_type(instance().exports.i32_i32(7)) == (7, int)

def test_call_i64_i64():
    assert value_with_type(instance().exports.i64_i64(7)) == (7, int)

def test_call_f32_f32():
    assert value_with_type(instance().exports.f32_f32(7.)) == (7., float)

def test_call_f64_f64():
    assert value_with_type(instance().exports.f64_f64(7.)) == (7., float)

def test_call_i32_i64_f32_f64_f64():
    assert round(instance().exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6), 6) == (
        1 + 2 + 3.4 + 5.6
    )

def test_call_bool_casted_to_i32():
    assert value_with_type(instance().exports.bool_casted_to_i32()) == (1, int)

def test_call_string():
    assert instance().exports.string() == 1048576

def test_call_void():
    assert instance().exports.void() == None
