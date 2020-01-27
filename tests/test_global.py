from wasmer import Instance
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/global.wasm', 'rb').read()

def test_global_mutable():
    globals = Instance(TEST_BYTES).globals

    assert globals.x.mutable == True
    assert globals.y.mutable == True
    assert globals.z.mutable == False

def test_global_read_write():
    y = Instance(TEST_BYTES).globals.y

    assert y.value == 7

    y.value = 8

    assert y.value == 8

def test_global_read_write_and_exported_functions():
    instance = Instance(TEST_BYTES)
    exports = instance.exports
    x = instance.globals.x

    assert x.value == 0
    assert exports.get_x() == 0

    x.value = 1

    assert x.value == 1
    assert exports.get_x() == 1

    exports.increment_x()

    assert x.value == 2
    assert exports.get_x() == 2

def test_global_read_write_constant():
    z = Instance(TEST_BYTES).globals.z

    assert z.value == 42

    with pytest.raises(RuntimeError) as context_manager:
        z.value = 153

    exception = context_manager.value
    assert str(exception) == (
        'The global variable `z` is not mutable, cannot set a new value.'
    )
