import os
import wasmer

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/../tests/tests.wasm', 'rb').read()

def test_benchmark_memory_view_int8_get(benchmark):
    memory = wasmer.Instance(TEST_BYTES).memory.uint8_view()

    def bench():
        _ = memory[0:255]

    benchmark(bench)

def test_benchmark_memory_view_memoryview_get(benchmark):
    memory = memoryview(wasmer.Instance(TEST_BYTES).memory.buffer)

    def bench():
        _ = memory[0:255]

    benchmark(bench)

def test_benchmark_memory_view_bytearray_get(benchmark):
    memory = bytearray(wasmer.Instance(TEST_BYTES).memory.buffer)

    def bench():
        _ = memory[0:255]

    benchmark(bench)
