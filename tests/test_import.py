from wasmer import Module
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/imported_function.wasm', 'rb').read()

def test_import_object_empty():
    import_object = Module(TEST_BYTES).generate_import_object()

    assert len(import_object.import_descriptors()) == 0

def test_import_object_extend():
    import_object = Module(TEST_BYTES).generate_import_object()
    import_object.extend({
        "env": {
            "sum": lambda x, y: x + y,
        }
    })

    import_descriptors = import_object.import_descriptors()

    assert len(import_descriptors) == 1
