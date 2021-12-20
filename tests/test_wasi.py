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

    import_object = env.generate_import_object(Store(), wasi.Version.LATEST)
    assert isinstance(import_object, dict)
    assert sorted(import_object['wasi_snapshot_preview1'].keys()) == [
        'args_get',
        'args_sizes_get',
        'clock_res_get',
        'clock_time_get',
        'environ_get',
        'environ_sizes_get',
        'fd_advise',
        'fd_allocate',
        'fd_close',
        'fd_datasync',
        'fd_fdstat_get',
        'fd_fdstat_set_flags',
        'fd_fdstat_set_rights',
        'fd_filestat_get',
        'fd_filestat_set_size',
        'fd_filestat_set_times',
        'fd_pread',
        'fd_prestat_dir_name',
        'fd_prestat_get',
        'fd_pwrite',
        'fd_read',
        'fd_readdir',
        'fd_renumber',
        'fd_seek',
        'fd_sync',
        'fd_tell',
        'fd_write',
        'path_create_directory',
        'path_filestat_get',
        'path_filestat_set_times',
        'path_link',
        'path_open',
        'path_readlink',
        'path_remove_directory',
        'path_rename',
        'path_symlink',
        'path_unlink_file',
        'poll_oneoff',
        'proc_exit',
        'proc_raise',
        'random_get',
        'sched_yield',
        'sock_recv',
        'sock_send',
        'sock_shutdown'
    ]

def test_wasi_env_memory():
    store = Store()
    wasi_env = wasi.StateBuilder("foo").finalize()
    import_object = wasi_env.generate_import_object(store, wasi.Version.LATEST)

    instance = Instance(Module(store, TEST_BYTES), import_object)

def test_wasi():
    store = Store()
    wasi_env = \
        wasi.StateBuilder("test-program"). \
            argument("--foo"). \
            environments({"ABC": "DEF", "X": "YZ"}). \
            map_directory("the_host_current_dir", "."). \
            finalize()
    import_object = wasi_env.generate_import_object(store, wasi.Version.LATEST)

    instance = Instance(Module(store, TEST_BYTES), import_object)
    instance.exports._start()
