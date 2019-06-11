import wasmer
from wasmer import Module
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def test_validate():
    assert Module.validate(TEST_BYTES)

def test_validate_invalid():
    assert not Module.validate(INVALID_TEST_BYTES)

def test_compile():
    assert isinstance(Module(TEST_BYTES), Module)

def test_failed_to_compile():
    with pytest.raises(RuntimeError) as context_manager:
        Module(INVALID_TEST_BYTES)

    exception = context_manager.value
    assert str(exception) == (
        'Failed to compile the module:\n    Validation error "Invalid type"'
    )

def test_instantiate():
    assert Module(TEST_BYTES).instantiate().exports.sum(1, 2) == 3
