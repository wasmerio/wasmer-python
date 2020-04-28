from wasmer import Module, WasiVersion, ImportObject
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/wasi.wasm', 'rb').read()
module = Module(wasm_bytes)

import_object = module.generate_wasi_import_object(
    module.wasi_version(False),
    ["wasi_test_program", "--help"],
    {
        "COLOR": "true",
        "APP_SHOULD_LOG": "false",
    },
    [],
    {
        "the_host_current_dir": ".",
    }
)

def host_print(ptr: int, len: int):
    print("host_print: {} {}".format(ptr, len))

import_object.extend({
    "env": {
        "host_print": host_print
    }
})

instance = module.instantiate(import_object)
instance.exports._start()
