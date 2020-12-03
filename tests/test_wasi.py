from wasmer import wasi, Store, ImportObject, Module, Instance
from enum import IntEnum
import os
import pytest
import subprocess
import sys

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/wasi.wasm', 'rb').read()

def test_wasi_version():
    assert issubclass(wasi.Version, IntEnum)
    assert len(wasi.Version) == 3
    assert wasi.Version.LATEST == 1
    assert wasi.Version.SNAPSHOT0 == 2
    assert wasi.Version.SNAPSHOT1 == 3

def test_wasi_get_version():
    assert wasi.get_version(Module(Store(), TEST_BYTES), strict=True) == wasi.Version.SNAPSHOT1

def test_wasi_state_builder():
    state_builder = \
        wasi.StateBuilder("test-program"). \
            argument("--foo"). \
            environments({"ABC": "DEF", "X": "YZ"}). \
            map_directory("the_host_current_dir", ".")   

    assert isinstance(state_builder, wasi.StateBuilder)

def test_wasi_env():
    assert isinstance(wasi.StateBuilder("foo").finalize(), wasi.Environment)

def test_wasi_import_object():
    env = wasi.StateBuilder("foo").finalize()

    assert isinstance(env.generate_import_object(Store(), wasi.Version.LATEST), ImportObject)

def test_wasi_env_memory():
    store = Store()
    wasi_env = wasi.StateBuilder("foo").finalize()
    import_object = wasi_env.generate_import_object(store, wasi.Version.LATEST)

    instance = Instance(Module(store, TEST_BYTES), import_object)

def test_wasi():
    python = sys.executable
    result = subprocess.check_output(
        [
            python,
            '-c',
            'from wasmer import wasi, Store, Module, Instance; \
            store = Store(); \
            module = Module(store, open("tests/wasi.wasm", "rb").read()); \
            wasi_version = wasi.get_version(module, strict=True); \
            wasi_env = wasi.StateBuilder("test-program").argument("--foo").environments({"ABC": "DEF", "X": "YZ"}).map_directory("the_host_current_dir", ".").finalize(); \
            import_object = wasi_env.generate_import_object(store, wasi_version); \
            instance = Instance(module, import_object); \
            instance.exports._start()'
        ]
    )

    assert result == b'Found program name: `test-program`\n\
Found 1 arguments: --foo\n\
Found 2 environment variables: ABC=DEF, X=YZ\n\
Found 1 preopened directories: DirEntry("/the_host_current_dir")\n'
