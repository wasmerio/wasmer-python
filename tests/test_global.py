from wasmer import Instance, Module, Store, Global, GlobalType, Type, Value
import pytest

TEST_BYTES = """
(module
  (global $x (export "x") (mut i32) (i32.const 0))
  (global $y (export "y") (mut i32) (i32.const 7))
  (global $z (export "z") i32 (i32.const 42))
 
  (func (export "get_x") (result i32)
    (global.get $x))
 
  (func (export "increment_x")
    (global.set $x
      (i32.add (global.get $x) (i32.const 1)))))
"""

def instance():
    return Instance(Module(Store(), TEST_BYTES))

def test_constructor():
    store = Store()
    global_ = Global(store, Value.i32(42))

    assert global_.value == 42

    type = global_.type

    assert type.type == Type.I32
    assert type.mutable == False

    global_ = Global(store, Value.i64(153), mutable=False)

    assert global_.value == 153

    type = global_.type

    assert type.type == Type.I64
    assert type.mutable == False

def test_constructor_mutable():
    store = Store()
    global_ = Global(store, Value.i32(42), mutable=True)

    assert global_.value == 42

    type = global_.type

    assert type.type == Type.I32
    assert type.mutable == True

    global_.value = 153

    assert global_.value == 153

def test_export():
    assert isinstance(instance().exports.x, Global)

def test_type():
    type = instance().exports.x.type

    assert type.type == Type.I32
    assert type.mutable == True
    assert str(type) == 'GlobalType(type: I32, mutable: true)'

def test_global_mutable():
    exports = instance().exports

    assert exports.x.mutable == True
    assert exports.y.mutable == True
    assert exports.z.mutable == False

def test_global_read_write():
    y = instance().exports.y

    assert y.value == 7

    y.value = 8

    assert y.value == 8

def test_global_read_write_and_exported_functions():
    exports = instance().exports
    x = exports.x

    assert x.value == 0
    assert exports.get_x() == 0

    x.value = 1

    assert x.value == 1
    assert exports.get_x() == 1

    exports.increment_x()

    assert x.value == 2
    assert exports.get_x() == 2

def test_global_read_write_constant():
    z = instance().exports.z

    assert z.value == 42

    with pytest.raises(RuntimeError) as context_manager:
        z.value = 153

    exception = context_manager.value
    assert str(exception) == (
        'The global variable is not mutable, cannot set a new value'
    )
