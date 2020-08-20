from wasmer import Type
from enum import IntEnum

def test_type():
    assert issubclass(Type, IntEnum)
    assert len(Type) == 7
    assert Type.I32 == 1
    assert Type.I64 == 2
    assert Type.F32 == 3
    assert Type.F64 == 4
    assert Type.V128 == 5
    assert Type.EXTERN_REF == 6
    assert Type.FUNC_REF == 7
