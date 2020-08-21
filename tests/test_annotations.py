#from inspect import Parameter, Signature, signature
#from wasmer import Instance, Memory, Module, Value, ImportObject, Wasi
#
#def test_instance():
#    # .__init__
#    assert Instance.__text_signature__ == "(bytes, imported_functions={})"
#    assert signature(Instance) == Signature(
#        [
#            Parameter("bytes", Parameter.POSITIONAL_OR_KEYWORD),
#            Parameter(
#                "imported_functions", Parameter.POSITIONAL_OR_KEYWORD, default={}
#            ),
#        ]
#    )
#    # .resolve_exported_function
#    assert Instance.resolve_exported_function.__text_signature__ == "($self, index)"
#    assert signature(Instance.resolve_exported_function) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("index", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#
#def test_memory():
#    def helper_offset(func):
#        assert func.__text_signature__ == "($self, offset=0)"
#        assert signature(func) == Signature(
#            [
#                Parameter("self", Parameter.POSITIONAL_ONLY),
#                Parameter("offset", Parameter.POSITIONAL_OR_KEYWORD, default=0),
#            ]
#        )
#
#    # .uint8_view
#    helper_offset(Memory.uint8_view)
#    # .int8_view
#    helper_offset(Memory.int8_view)
#    # .uint16_view
#    helper_offset(Memory.uint16_view)
#    # .int16_view
#    helper_offset(Memory.int16_view)
#    # .uint32_view
#    helper_offset(Memory.uint32_view)
#    # .int32_view
#    helper_offset(Memory.int32_view)
#    # .grow
#    assert Memory.grow.__text_signature__ == "($self, number_of_pages)"
#    assert signature(Memory.grow) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("number_of_pages", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#
#def test_module():
#    # .__init__
#    helper_single_param(Module, param_name="bytes")
#    # .instantiate
#    assert Module.instantiate.__text_signature__ == "($self, import_object={})"
#    assert signature(Module.instantiate) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("import_object", Parameter.POSITIONAL_OR_KEYWORD, default={}),
#        ]
#    )
#    # .custom_section
#    assert Module.custom_section.__text_signature__ == "($self, name, index=0)"
#    assert signature(Module.custom_section) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("name", Parameter.POSITIONAL_OR_KEYWORD),
#            Parameter("index", Parameter.POSITIONAL_OR_KEYWORD, default=0),
#        ]
#    )
#    # .serialize
#    helper_self(Module.serialize)
#    # .deserialize
#    helper_single_param(Module.deserialize, param_name="bytes")
#    # .generate_import_object
#    helper_self(Module.generate_import_object)
#    # .wasi_version
#    assert Module.wasi_version.__text_signature__ == "($self, strict=False)"
#    assert signature(Module.wasi_version) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("strict", Parameter.POSITIONAL_OR_KEYWORD, default=False),
#        ]
#    )
#    # .validate
#    helper_single_param(Module.validate, param_name="bytes")
#
#def test_value():
#    # .i32
#    helper_single_param(Value.i32, param_name="value")
#    # .i64
#    helper_single_param(Value.i64, param_name="value")
#    # .f32
#    helper_single_param(Value.f32, param_name="value")
#    # .f64
#    helper_single_param(Value.f64, param_name="value")
#    # .v128
#    helper_single_param(Value.v128, param_name="value")
#
#def test_import_object():
#    # .extend
#    assert ImportObject.extend.__text_signature__ == "($self, imported_functions)"
#    assert signature(ImportObject.extend) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("imported_functions", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .import_descriptors
#    helper_self(ImportObject.import_descriptors)
#
#def test_wasi():
#    # .__init__
#    assert Wasi.__init__.__text_signature__ == "($self, /, *args, **kwargs)"
#    # .arguments
#    assert Wasi.arguments.__text_signature__ == "($self, arguments)"
#    assert signature(Wasi.arguments) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("arguments", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .argument
#    assert Wasi.argument.__text_signature__ == "($self, argument)"
#    assert signature(Wasi.argument) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("argument", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .environments
#    assert Wasi.environments.__text_signature__ == "($self, environments)"
#    assert signature(Wasi.environments) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("environments", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .environment
#    assert Wasi.environment.__text_signature__ == "($self, key, value)"
#    assert signature(Wasi.environment) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("key", Parameter.POSITIONAL_OR_KEYWORD),
#            Parameter("value", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .preopen_directories
#    assert Wasi.preopen_directories.__text_signature__ == "($self, preopen_directories)"
#    assert signature(Wasi.preopen_directories) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("preopen_directories", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .preopen_directory
#    assert Wasi.preopen_directory.__text_signature__ == "($self, preopen_directory)"
#    assert signature(Wasi.preopen_directory) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("preopen_directory", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .map_directories
#    assert Wasi.map_directories.__text_signature__ == "($self, map_directories)"
#    assert signature(Wasi.map_directories) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("map_directories", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .map_directory
#    assert Wasi.map_directory.__text_signature__ == "($self, alias, directory)"
#    assert signature(Wasi.map_directory) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("alias", Parameter.POSITIONAL_OR_KEYWORD),
#            Parameter("directory", Parameter.POSITIONAL_OR_KEYWORD),
#        ]
#    )
#    # .generate_import_object_for_module
#    assert Wasi.generate_import_object_for_module.__text_signature__ == "($self, module, version=0)"
#    assert signature(Wasi.generate_import_object_for_module) == Signature(
#        [
#            Parameter("self", Parameter.POSITIONAL_ONLY),
#            Parameter("module", Parameter.POSITIONAL_OR_KEYWORD),
#            Parameter("version", Parameter.POSITIONAL_OR_KEYWORD, default=0),
#        ]
#    )
#
## helper functions
##
#def helper_single_param(func, *, param_name: str):
#    """Check text_signature and signature for function with one parameter (except `self`)"""
#
#    assert func.__text_signature__ == "({})".format(param_name)
#    assert signature(func) == Signature(
#        [Parameter(param_name, Parameter.POSITIONAL_OR_KEYWORD)]
#    )
#
#
#def helper_self(func):
#    assert func.__text_signature__ == "($self)"
#    assert signature(func) == Signature(
#        [Parameter("self", Parameter.POSITIONAL_ONLY)]
#    )
