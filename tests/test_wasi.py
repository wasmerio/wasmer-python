from wasmer import Module, WasiVersion, Wasi, ImportKind, Features
from enum import IntEnum
import inspect
import os
import pytest
import subprocess
import sys

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/wasi.wasm', 'rb').read()

def test_wasi_version():
    assert issubclass(WasiVersion, IntEnum)
    assert len(WasiVersion) == 3
    assert WasiVersion.Snapshot0 == 1
    assert WasiVersion.Snapshot1 == 2
    assert WasiVersion.Latest == 3

def test_wasi_import_object():
    module = Module(TEST_BYTES)
    import_object = Wasi('test-program').generate_import_object_for_module(module)
    descriptors = sorted(import_object.import_descriptors(), key=lambda item: item['name'])

    assert descriptors == [
        {'kind': ImportKind.FUNCTION, 'name': 'args_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'args_sizes_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'clock_res_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'clock_time_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'environ_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'environ_sizes_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_advise', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_allocate', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_close', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_datasync', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_fdstat_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_fdstat_set_flags', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_fdstat_set_rights', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_filestat_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_filestat_set_size', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_filestat_set_times', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_pread', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_prestat_dir_name', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_prestat_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_pwrite', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_read', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_readdir', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_renumber', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_seek', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_sync', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_tell', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'fd_write', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_create_directory', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_filestat_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_filestat_set_times', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_link', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_open', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_readlink', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_remove_directory', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_rename', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_symlink', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'path_unlink_file', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'poll_oneoff', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'proc_exit', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'proc_raise', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'random_get', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'sched_yield', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'sock_recv', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'sock_send', 'namespace': 'wasi_snapshot_preview1'},
        {'kind': ImportKind.FUNCTION, 'name': 'sock_shutdown', 'namespace': 'wasi_snapshot_preview1'}
    ]

def test_wasi_version_from_module():
    module = Module(TEST_BYTES)

    assert module.is_wasi_module == True
    assert module.wasi_version() == WasiVersion.Snapshot1
    assert module.wasi_version(True) == WasiVersion.Snapshot1

@pytest.mark.skipif(Features.wasi() == False, reason='WASI is not supported on aarch64 for the moment')
def test_wasi():
    python = sys.executable
    result = subprocess.check_output(
        [
            python,
            '-c',
            'from wasmer import Module, Wasi; \
            module = Module(open("tests/wasi.wasm", "rb").read()); \
            import_object = Wasi("test-program").argument("--foo").environments({"ABC": "DEF", "X": "YZ"}).map_directory("the_host_current_dir", ".").generate_import_object_for_module(module); \
            instance = module.instantiate(import_object); \
            instance.exports._start()'
        ]
    )

    assert result == b'Found program name: `test-program`\n\
Found 1 arguments: --foo\n\
Found 2 environment variables: ABC=DEF, X=YZ\n\
Found 1 preopened directories: DirEntry("/the_host_current_dir")\n'
