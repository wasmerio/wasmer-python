import wasmer
from wasmer import Module, ExportKind
from enum import IntEnum
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

def test_export_kind():
    assert issubclass(ExportKind, IntEnum)
    assert len(ExportKind) == 4
    assert ExportKind.FUNCTION == 1
    assert ExportKind.MEMORY == 2
    assert ExportKind.GLOBAL == 3
    assert ExportKind.TABLE == 4

def test_exports():
    assert Module(TEST_BYTES).exports == [
        {
            "name": "memory",
            "kind": ExportKind.MEMORY,
        },
        {
            "name": "__heap_base",
            "kind": ExportKind.GLOBAL,
        },
        {
            "name": "__data_end",
            "kind": ExportKind.GLOBAL,
        },
        {
            "name": "sum",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "arity_0",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "i32_i32",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "i64_i64",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "f32_f32",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "f64_f64",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "i32_i64_f32_f64_f64",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "bool_casted_to_i32",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "string",
            "kind": ExportKind.FUNCTION,
        },
        {
            "name": "void",
            "kind": ExportKind.FUNCTION,
        },
    ]

def test_serialize():
    assert type(Module(TEST_BYTES).serialize()) == bytes

def test_deserialize():
    serialized_module = Module(TEST_BYTES).serialize()
    module = Module.deserialize(serialized_module)
    del serialized_module

    assert module.instantiate().exports.sum(1, 2) == 3
