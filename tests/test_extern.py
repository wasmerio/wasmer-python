from wasmer import ExternType
from enum import IntEnum

def test_extern_type():
    assert issubclass(ExternType, IntEnum)
    assert len(ExternType) == 4
    assert ExternType.FUNCTION == 1
    assert ExternType.GLOBAL == 2
    assert ExternType.TABLE == 3
    assert ExternType.MEMORY == 4
