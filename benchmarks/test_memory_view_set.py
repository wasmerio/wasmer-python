import os
import wasmer

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/../tests/tests.wasm', 'rb').read()

N = 5000

def test_benchmark_memory_view_set_sequence_with_slice_assignment(benchmark):
    memory = wasmer.Instance(TEST_BYTES).memory.uint8_view()

    def bench():
        memory[0:255] = range(0, 255)

    benchmark(bench)


def test_benchmark_memory_view_set_sequence_with_a_loop(benchmark):
    memory = wasmer.Instance(TEST_BYTES).memory.uint8_view()

    def bench():
        for nth in range(0, 255):
            memory[nth] = nth

    benchmark(bench)
