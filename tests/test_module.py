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
