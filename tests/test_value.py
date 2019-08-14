from wasmer import Value
import inspect
import pytest

def test_is_a_class():
    assert inspect.isclass(Value)

@pytest.mark.xfail()
def test_cannot_construct():
    Value()

def test_i32():
    assert repr(Value.i32(42)) == 'I32(42)'

def test_i64():
    assert repr(Value.i64(42)) == 'I64(42)'

def test_f32():
    assert repr(Value.f32(4.2)) == 'F32(4.2)'

def test_f32_auto_cast():
    assert repr(Value.f32(42)) == 'F32(42.0)'

def test_f64():
    assert repr(Value.f64(4.2)) == 'F64(4.2)'

def test_f64_auto_cast():
    assert repr(Value.f64(42)) == 'F64(42.0)'

def test_v128():
    assert repr(Value.v128(340282366920938463463374607431768211455)) == 'V128(340282366920938463463374607431768211455)'
