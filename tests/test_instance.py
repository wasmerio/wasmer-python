import wasmer
from wasmer import Instance, Module, Store, Exports
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

def test_version():
    assert isinstance(wasmer.__version__, str)

def test_core_version():
    assert isinstance(wasmer.__core_version__, str)

def test_new():
    assert isinstance(Instance(Module(Store(), TEST_BYTES)), Instance)

def test_exports():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert isinstance(instance.exports, Exports)

def test_exports_len():
    instance = Instance(Module(Store(), TEST_BYTES))

    assert len(instance.exports) == 13

def test_export_does_not_exist():
    with pytest.raises(LookupError) as context_manager:
        Instance(Module(Store(), TEST_BYTES)).exports.foo

    exception = context_manager.value
    assert str(exception) == 'Export `foo` does not exist.'

#
#def test_memory_view():
#    assert isinstance(Instance(TEST_BYTES).memory.uint8_view(), Uint8Array)
#
#@pytest.mark.skipif(sys.version_info < (3, 6), reason='require Python 3.6+ to run')
#def test_getfullargspec():
#    instance = Instance(TEST_BYTES)
#    assert instance.exports.sum.getfullargspec == inspect.FullArgSpec(
#        args=['x0', 'x1'],
#        varargs=None,
#        varkw=None,
#        defaults=None,
#        kwonlyargs=None,
#        kwonlydefaults=None,
#        annotations={'x0': Type.I32, 'x1': Type.I32, 'return': Type.I32}
#    )
#
#def test_resolve_exported_function():
#    assert Instance(TEST_BYTES).resolve_exported_function(0) == "sum"
