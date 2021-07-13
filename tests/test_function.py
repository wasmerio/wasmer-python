import wasmer
from wasmer import Instance, Module, Store, Function, FunctionType, Type, ImportObject
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def instance():
    return Instance(Module(Store(), TEST_BYTES))

def value_with_type(value):
    return (value, type(value))

def test_constructor_with_annotated_function():
    store = Store()

    def sum(a: int, b: 'i32', c: 'I32', d: 'i64', e: 'I64', f: float, g: 'f32', h: 'F32', i: 'f64', j: 'F64') -> int:
        return a + b

    function = Function(store, sum)
    function_type = function.type

    assert function_type.params == [Type.I32, Type.I32, Type.I32, Type.I64, Type.I64, Type.F32, Type.F32, Type.F32, Type.F64, Type.F64]
    assert function_type.results == [Type.I32]

    def return_none(a: int) -> None:
        pass

    function = Function(store, return_none)
    function_type = function.type

    assert function_type.params == [Type.I32]
    assert function_type.results == []

    def take_none(a: None):
        pass

    with pytest.raises(RuntimeError) as context_manager:
        Function(store, take_none)

    exception = context_manager.value
    assert str(exception) == 'Variable `a` cannot have type `None`'

    def tuple(a: int) -> (int, int):
        return (a, a)

    function = Function(store, tuple)
    function_type = function.type

    assert function_type.params == [Type.I32]
    assert function_type.results == [Type.I32, Type.I32]

def test_constructor_with_blank_function():
    def sum(x, y):
        return x + y

    store = Store()
    function = Function(store, sum, FunctionType([Type.I32, Type.I32], [Type.I32]))

def test_export():
    assert isinstance(instance().exports.sum, Function)

def test_type():
    type = instance().exports.sum.type

    assert isinstance(type, FunctionType)
    assert type.params == [Type.I32, Type.I32]
    assert type.results == [Type.I32]
    assert str(type) == 'FunctionType(params: [I32, I32], results: [I32])'

def test_basic_sum():
    assert value_with_type(instance().exports.sum(1, 2)) == (3, int)

def test_call_arity_0():
    assert value_with_type(instance().exports.arity_0()) == (42, int)

def test_call_i32_i32():
    assert value_with_type(instance().exports.i32_i32(7)) == (7, int)

def test_call_i64_i64():
    assert value_with_type(instance().exports.i64_i64(7)) == (7, int)

def test_call_f32_f32():
    assert value_with_type(instance().exports.f32_f32(7.)) == (7., float)

def test_call_f64_f64():
    assert value_with_type(instance().exports.f64_f64(7.)) == (7., float)

def test_call_i32_i64_f32_f64_f64():
    assert round(instance().exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6), 6) == (
        1 + 2 + 3.4 + 5.6
    )

def test_call_bool_casted_to_i32():
    assert value_with_type(instance().exports.bool_casted_to_i32()) == (1, int)

def test_call_string():
    assert instance().exports.string() == 1048576

def test_call_void():
    assert instance().exports.void() == None

def test_early_exit():
    store = Store()
    module = Module(
        store,
        """
        (module
          (type $run_t (func (param i32 i32) (result i32)))
          (type $early_exit_t (func (param) (result)))

          (import "env" "early_exit" (func $early_exit (type $early_exit_t)))

          (func $run (type $run_t) (param $x i32) (param $y i32) (result i32)
            (call $early_exit)
            (i32.add
                local.get $x
                local.get $y))

          (export "run" (func $run)))
        """
    )

    def early_exit():
        raise Exception('oops')

    import_object = ImportObject()
    import_object.register(
        "env",
        {
            "early_exit": Function(store, early_exit),
        }
    )
    instance = Instance(module, import_object)

    try:
        instance.exports.run(1, 2)
    except RuntimeError as err:
        assert 'oops' in str(err)
    else:
        assert False

def test_return_multiple_values():
    store = Store()
    module = Module(
        store,
        """
        (module
          (type $swap_t (func (param i32 i64) (result i64 i32)))
          (func $swap_f (type $swap_t) (param $x i32) (param $y i64) (result i64 i32)
            local.get $y
            local.get $x)
          (export "swap" (func $swap_f)))
        """
    )
    instance = Instance(module)

    assert instance.exports.swap(41, 42) == (42, 41)

def test_return_multiple_values_from_host_function():
    store = Store()
    module = Module(
        store,
        """
        (module
          (type $swap_t (func (param i32 i64) (result i64 i32)))
          (type $test_t (func (param i32 i64) (result i64 i32)))

          (import "env" "swap" (func $swap (type $swap_t)))

          (func $test (type $test_t) (param $x i32) (param $y i64) (result i64 i32)
            local.get $x
            local.get $y
            call $swap)
          (export "test" (func $test)))
        """
    )

    def swap(x: 'i32', y: 'i64') -> ('i64', 'i32'):
        return (y, x)

    import_object = ImportObject()
    import_object.register(
        "env",
        {
            "swap": Function(store, swap),
        }
    )
    instance = Instance(module, import_object)

    assert instance.exports.test(41, 42) == (42, 41)
