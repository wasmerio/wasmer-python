from inspect import (
    Parameter,
    Signature as Signature_Class,
    signature,
)

from wasmer import Instance, Memory, Module, Value


def test_instance():
    # .__init__
    assert Instance.__text_signature__ == "(bytes, imported_functions={})"
    assert signature(Instance) == Signature_Class(
        [
            Parameter("bytes", Parameter.POSITIONAL_OR_KEYWORD),
            Parameter(
                "imported_functions", Parameter.POSITIONAL_OR_KEYWORD, default={}
            ),
        ]
    )
    # .resolve_exported_function
    assert Instance.resolve_exported_function.__text_signature__ == "($self, index)"
    assert signature(Instance.resolve_exported_function) == Signature_Class(
        [
            Parameter("self", Parameter.POSITIONAL_ONLY),
            Parameter("index", Parameter.POSITIONAL_OR_KEYWORD),
        ]
    )


def test_memory():
    # .uint8_view
    helper_offset(Memory.uint8_view)
    # .int8_view
    helper_offset(Memory.int8_view)
    # .uint16_view
    helper_offset(Memory.uint16_view)
    # .int16_view
    helper_offset(Memory.int16_view)
    # .uint32_view
    helper_offset(Memory.uint32_view)
    # .int32_view
    helper_offset(Memory.int32_view)
    # .grow
    assert Memory.grow.__text_signature__ == "($self, number_of_pages)"
    assert signature(Memory.grow) == Signature_Class(
        [
            Parameter("self", Parameter.POSITIONAL_ONLY),
            Parameter("number_of_pages", Parameter.POSITIONAL_OR_KEYWORD),
        ]
    )


def test_module():
    # .__init__
    helper_single_param(Module, param_name="bytes")
    # .instantiate
    helper_self(Module.instantiate)
    # .custom_section
    assert Module.custom_section.__text_signature__ == "($self, name, index=0)"
    assert signature(Module.custom_section) == Signature_Class(
        [
            Parameter("self", Parameter.POSITIONAL_ONLY),
            Parameter("name", Parameter.POSITIONAL_OR_KEYWORD),
            Parameter("index", Parameter.POSITIONAL_OR_KEYWORD, default=0),
        ]
    )
    # .serialize
    helper_self(Module.serialize)
    # .deserialize
    helper_single_param(Module.deserialize, param_name="bytes")
    # .validate
    helper_single_param(Module.validate, param_name="bytes")


def test_value():
    # .i32
    helper_single_param(Value.i32, param_name="value")
    # .i64
    helper_single_param(Value.i64, param_name="value")
    # .f32
    helper_single_param(Value.f32, param_name="value")
    # .f64
    helper_single_param(Value.f64, param_name="value")
    # .v128
    helper_single_param(Value.v128, param_name="value")


# helper functions
#
def helper_single_param(func, *, param_name: str):
    """Check text_signature and signature for function with one parameter (except `self`)"""

    assert func.__text_signature__ == "({})".format(param_name)
    assert signature(func) == Signature_Class(
        [Parameter(param_name, Parameter.POSITIONAL_OR_KEYWORD)]
    )


def helper_self(func):
    assert func.__text_signature__ == "($self)"
    assert signature(func) == Signature_Class(
        [Parameter("self", Parameter.POSITIONAL_ONLY)]
    )


def helper_offset(func):
    assert func.__text_signature__ == "($self, offset=0)"
    assert signature(func) == Signature_Class(
        [
            Parameter("self", Parameter.POSITIONAL_ONLY),
            Parameter("offset", Parameter.POSITIONAL_OR_KEYWORD, default=0),
        ]
    )
