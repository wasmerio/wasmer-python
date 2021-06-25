from wasmer import engine, Store, Module, Instance
import itertools
import os
import platform
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def test_store_defaults():
    store = Store()

    assert store.engine_name == 'universal'
    assert store.compiler_name == 'cranelift'

def test_store_universal():
    store = Store(engine.Universal())

    assert store.engine_name == 'universal'
    assert store.compiler_name == None

def test_store_dylib():
    store = Store(engine.Dylib())

    assert store.engine_name == 'dylib'
    assert store.compiler_name == None

def test_store_deprecated_jit():
    store = Store(engine.JIT())

    assert store.engine_name == 'universal'
    assert store.compiler_name == None

def test_store_deprecated_native():
    store = Store(engine.Native())

    assert store.engine_name == 'dylib'
    assert store.compiler_name == None

#@pytest.mark.skipif(platform.system() == 'Windows', reason='Wasmer (`master`) has some troubles with JIT on Windows for the moment.')
#def test_store_with_various_engines_and_compilers():
#    import wasmer_compiler_llvm
#
#    engines = [
#        engine.Universal,
#        engine.Dylib
#    ]
#    compilers = [
#        None,
#        wasmer_compiler_cranelift.Compiler,
#        wasmer_compiler_llvm.Compiler,
#        wasmer_compiler_singlepass.Compiler
#    ]
#    results = [
#        ('universal', None),
#        ('universal', 'cranelift'),
#        ('universal', 'llvm'),
#        ('universal', 'singlepass'),
#        ('dylib', None),
#        ('dylib', 'cranelift'),
#        ('dylib', 'llvm'),
#        ('dylib', 'singlepass'),
#    ]
#
#    for ((engine_, compiler), expected) in itertools.zip_longest(itertools.product(engines, compilers), results):
#        store = Store(engine_(compiler))
#
#        assert store.engine_name == expected[0]
#        assert store.compiler_name == expected[1]
#
#        if compiler != None:
#            module = Module(store, TEST_BYTES)
#            instance = Instance(module)
#
#            assert instance.exports.sum(1, 2)
