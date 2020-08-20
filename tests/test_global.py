from wasmer import Instance, Module, Store
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

#def test_global_read_write_and_exported_functions():
#    instance = Instance(TEST_BYTES)
#    exports = instance.exports
#    x = instance.globals.x
#
#    assert x.value == 0
#    assert exports.get_x() == 0
#
#    x.value = 1
#
#    assert x.value == 1
#    assert exports.get_x() == 1
#
#    exports.increment_x()
#
#    assert x.value == 2
#    assert exports.get_x() == 2
#
#def test_global_read_write_constant():
#    z = Instance(TEST_BYTES).globals.z
#
#    assert z.value == 42
#
#    with pytest.raises(RuntimeError) as context_manager:
#        z.value = 153
#
#    exception = context_manager.value
#    assert str(exception) == (
#        'The global variable `z` is not mutable, cannot set a new value.'
#    )
