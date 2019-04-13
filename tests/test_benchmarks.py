from wasmer import Instance, Uint8MemoryView, Value, validate
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def test_sum_benchmark(benchmark):
    instance = Instance(TEST_BYTES)
    sum_func = instance.exports['sum']

    def bench():
        return sum_func(1, 2)

    assert benchmark(bench) == 3
