from wasmer import Store, Module, Instance, ImportObject, wasi
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))

store = Store()
module = Module(store, open(__dir__ + "/wasi.wasm", "rb").read())

wasi_version = wasi.get_version(module, strict=True)
wasi_env = \
    wasi.StateBuilder("wasi_test_program"). \
        argument("--test"). \
        environment("COLOR", "true"). \
        environment("APP_SHOULD_LOG", "false"). \
        map_directory("the_host_current_dir", "."). \
        finalize()

import_object = wasi_env.generate_import_object(store, wasi_version)

instance = Instance(module, import_object)

wasi_env.memory = instance.exports.memory

instance.exports._start()
