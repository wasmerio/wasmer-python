from wasmer import ImportObject, Store, Module, Instance, Function, Memory, MemoryType
import pytest

def test_constructor():
    import_object = ImportObject()

def test_contains_namespace():
    import_object = ImportObject()

    assert import_object.contains_namespace("foo") == False

def test_import_function():
    def sum(x: int, y: int) -> int:
        return x + y

    store = Store()
    module = Module(
        store,
        """
        (module
          (import "math" "sum" (func $sum (param i32 i32) (result i32)))
          (func (export "add_one") (param i32) (result i32)
            local.get 0
            i32.const 1
            call $sum))
        """
    )

    import_object = ImportObject()
    import_object.register(
        "math",
        {
            "sum": Function(store, sum)
        }
    )

    instance = Instance(module, import_object)

    assert instance.exports.add_one(1) == 2

def test_import_memory():
    store = Store()
    module = Module(
        store,
        """
        (module
          (import "env" "memory" (memory $memory 1))
          (func (export "increment")
            i32.const 0
            i32.const 0
            i32.load    ;; load 0
            i32.const 1
            i32.add     ;; add 1
            i32.store   ;; store at 0
            ))
        """
    )

    memory = Memory(store, MemoryType(1, shared=False))
    view = memory.uint8_view(offset=0)

    import_object = ImportObject()
    import_object.register(
        "env",
        {
            "memory": memory
        }
    )

    instance = Instance(module, import_object)

    assert view[0] == 0
    instance.exports.increment()
    assert view[0] == 1
    instance.exports.increment()
    assert view[0] == 2
