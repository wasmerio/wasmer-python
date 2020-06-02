import wasmer
from wasmer import Module, ExportKind, ImportKind, ImportObject
from enum import IntEnum
import inspect
import os
import pytest
import sys

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

def test_instantiate():
    assert Module(TEST_BYTES).instantiate().exports.sum(1, 2) == 3

def test_export_kind():
    assert issubclass(ExportKind, IntEnum)
    assert len(ExportKind) == 4
    assert ExportKind.FUNCTION == 1
    assert ExportKind.MEMORY == 2
    assert ExportKind.GLOBAL == 3
    assert ExportKind.TABLE == 4

def test_import_kind():
    assert issubclass(ImportKind, IntEnum)
    assert len(ImportKind) == 4
    assert ImportKind.FUNCTION == 1
    assert ImportKind.MEMORY == 2
    assert ImportKind.GLOBAL == 3
    assert ImportKind.TABLE == 4

def test_exports():
    assert Module(TEST_BYTES).exports == [
        {
            'name': 'memory',
            'kind': ExportKind.MEMORY,
        },
        {
            'name': '__heap_base',
            'kind': ExportKind.GLOBAL,
        },
        {
            'name': '__data_end',
            'kind': ExportKind.GLOBAL,
        },
        {
            'name': 'sum',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'arity_0',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'i32_i32',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'i64_i64',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'f32_f32',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'f64_f64',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'i32_i64_f32_f64_f64',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'bool_casted_to_i32',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'string',
            'kind': ExportKind.FUNCTION,
        },
        {
            'name': 'void',
            'kind': ExportKind.FUNCTION,
        },
    ]

@pytest.mark.skipif(sys.platform.startswith('win'), reason='https://github.com/wasmerio/wasmer/pull/1280/')
def test_imports():
    assert Module(open(here + '/imports.wasm', 'rb').read()).imports == [
        {
            'kind': ImportKind.FUNCTION,
            'namespace': 'ns',
            'name': 'f1',
        },
        {
            'kind': ImportKind.FUNCTION,
            'namespace': 'ns',
            'name': 'f2',
        },
        {
            'kind': ImportKind.MEMORY,
            'namespace': 'ns',
            'name': 'm1',
            'minimum_pages': 3,
            'maximum_pages': 4,
        },
        {
            'kind': ImportKind.GLOBAL,
            'namespace': 'ns',
            'name': 'g1',
            'mutable': False,
            'type': 'f32'
        },
        {
            'kind': ImportKind.TABLE,
            'namespace': 'ns',
            'name': 't1',
            'minimum_elements': 1,
            'maximum_elements': 2,
            'element_type': 'anyfunc',
        }
    ]

@pytest.mark.skipif(sys.platform.startswith('win'), reason='https://github.com/wasmerio/wasmer/pull/1280/')
def test_custom_section_names():
    assert sorted(Module(open(here + '/custom_sections.wasm', 'rb').read()).custom_section_names) == ['easter_egg', 'hello']

@pytest.mark.skipif(sys.platform.startswith('win'), reason='https://github.com/wasmerio/wasmer/pull/1280/')
def test_custom_section():
    module = Module(open(here + '/custom_sections.wasm', 'rb').read())
    assert module.custom_section('easter_egg') == b'Wasmer'
    assert module.custom_section('hello') == b'World!'
    assert module.custom_section('foo') == None

def test_serialize():
    assert type(Module(TEST_BYTES).serialize()) == bytes

def test_deserialize():
    serialized_module = Module(TEST_BYTES).serialize()
    module = Module.deserialize(serialized_module)
    del serialized_module

    assert module.instantiate().exports.sum(1, 2) == 3

def test_generate_import_object():
    module = Module(TEST_BYTES)
    import_object = module.generate_import_object()

    assert isinstance(import_object, ImportObject)
    assert len(import_object.import_descriptors()) == 0
