from wasmer import Instance, Uint8MemoryView, Int8MemoryView, Uint16MemoryView, Int16MemoryView, Uint32MemoryView, Int32MemoryView
import inspect
import os
import unittest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

class TestWasmMemoryView(unittest.TestCase):
    def test_is_a_class(self):
        self.assertTrue(inspect.isclass(Uint8MemoryView))
        self.assertTrue(inspect.isclass(Int8MemoryView))
        self.assertTrue(inspect.isclass(Uint16MemoryView))
        self.assertTrue(inspect.isclass(Int16MemoryView))
        self.assertTrue(inspect.isclass(Uint32MemoryView))
        self.assertTrue(inspect.isclass(Int32MemoryView))

    @unittest.expectedFailure
    def test_cannot_construct(self):
        self.assertIsInstance(Uint8MemoryView(0), Uint8MemoryView)

    def test_length(self):
        self.assertEqual(
            Instance(TEST_BYTES).uint8_memory_view().length(),
            1114112
        )

    def test_get(self):
        memory = Instance(TEST_BYTES).uint8_memory_view()
        index = 7
        value = 42
        memory.set(index, value)

        self.assertEqual(memory.get(index), value)

    def test_set_returns_none(self):
        self.assertEqual(
            Instance(TEST_BYTES).uint8_memory_view().set(7, 42),
            None
        )

    def test_hello_world(self):
        instance = Instance(TEST_BYTES)
        pointer = instance.call('string')
        memory = instance.uint8_memory_view(pointer)
        nth = 0
        string = ''

        while (0 != memory.get(nth)):
            string += chr(memory.get(nth))
            nth += 1

        self.assertEqual(string, 'Hello, World!')
