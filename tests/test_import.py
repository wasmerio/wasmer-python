from wasmer import Module, ImportKind, Features
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/imported_function.wasm', 'rb').read()

def test_import_object_empty():
    import_object = Module(TEST_BYTES).generate_import_object()

    assert len(import_object.import_descriptors()) == 0

@pytest.mark.skipif(Features.host_functions() == False, reason='Host functions are not supported for this platform')
def test_import_object_extend():
    import_object = Module(TEST_BYTES).generate_import_object()
    import_object.extend({
        "env": {
            "sum": lambda x, y: x + y,
        }
    })

    import_descriptors = import_object.import_descriptors()

    assert import_object.import_descriptors() == [
        {'kind': ImportKind.FUNCTION, 'name': 'sum', 'namespace': 'env'}
    ]
