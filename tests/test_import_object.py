from wasmer import ImportObject, Store, Module, Instance, Function
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
          (import "math" "sum" (func (param i32 i32) (result i32)))
          (func (export "add_one") (param i32) (result i32)
            local.get 0
            i32.const 1
            call 0))
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
