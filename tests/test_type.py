from wasmer import Type
from enum import IntEnum

def test_type():
    assert issubclass(Type, IntEnum)
    assert len(Type) == 5
    assert Type.I32 == 1
    assert Type.I64 == 2
    assert Type.F32 == 3
    assert Type.F64 == 4
    assert Type.V128 == 5
