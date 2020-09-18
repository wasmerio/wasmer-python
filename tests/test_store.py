from wasmer import engine, Store, Module, Instance
import wasmer_compiler_cranelift
#import wasmer_compiler_llvm
import wasmer_compiler_singlepass
import itertools
import os
import platform
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def test_store_defaults():
    store = Store()

    assert store.engine_name == 'jit'
    assert store.compiler_name == 'cranelift'

def test_store_with_various_engines_and_compilers():
    engines = [
        engine.JIT,
        engine.Native
    ]
    compilers = [
        None,
        wasmer_compiler_cranelift.Compiler,
        #wasmer_compiler_llvm.Compiler,
        wasmer_compiler_singlepass.Compiler
    ]
    results = [
        ('jit', None),
        ('jit', 'cranelift'),
        #('jit', 'llvm'),
        ('jit', 'singlepass'),
        ('native', None),
        ('native', 'cranelift'),
        #('native', 'llvm'),
        ('native', 'singlepass'),
    ]

    for ((engine_, compiler), expected) in itertools.zip_longest(itertools.product(engines, compilers), results):
        store = Store(engine_(compiler))

        assert store.engine_name == expected[0]
        assert store.compiler_name == expected[1]

        if compiler != None:
            module = Module(store, TEST_BYTES)
            instance = Instance(module)

            assert instance.exports.sum(1, 2)
