from wasmer import Module, ImportObject, WasiVersion, Wasi
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + "/wasi.wasm", "rb").read()
module = Module(wasm_bytes)

wasi = \
    Wasi("wasi_test_program"). \
        argument("--test"). \
        environment("COLOR", "true"). \
        environment("APP_SHOULD_LOG", "false"). \
        map_directory("the_host_current_dir", ".")

import_object = wasi.generate_import_object_for_module(module)

instance = module.instantiate(import_object)
instance.exports._start()
