from wasmer import Instance, Module, Store, Table, TableType, Type
import pytest

TEST_BYTES = """
(module
  (table (export "table") 0 funcref))
"""

def instance():
    return Instance(Module(Store(), TEST_BYTES))

def test_export():
    assert isinstance(instance().exports.table, Table)

def test_type():
    type = instance().exports.table.type

    assert type.type == Type.FUNC_REF
    assert type.minimum == 0
    assert type.maximum == None
    assert str(type) == 'TableType(type: FuncRef, minimum: 0, maximum: None)'

def test_size():
    assert instance().exports.table.size == 0
