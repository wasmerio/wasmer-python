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

    def test_bytes_per_element(self):
        self.assertEqual(Instance(TEST_BYTES).uint8_memory_view().bytes_per_element, 1)
        self.assertEqual(Instance(TEST_BYTES).int8_memory_view().bytes_per_element, 1)
        self.assertEqual(Instance(TEST_BYTES).uint16_memory_view().bytes_per_element, 2)
        self.assertEqual(Instance(TEST_BYTES).int16_memory_view().bytes_per_element, 2)
        self.assertEqual(Instance(TEST_BYTES).uint32_memory_view().bytes_per_element, 4)
        self.assertEqual(Instance(TEST_BYTES).int32_memory_view().bytes_per_element, 4)

    @unittest.expectedFailure
    def test_cannot_construct(self):
        self.assertIsInstance(Uint8MemoryView(0), Uint8MemoryView)

    def test_length(self):
        self.assertEqual(
            len(Instance(TEST_BYTES).uint8_memory_view()),
            1114112
        )

    #def test_get(self):
    #    memory = Instance(TEST_BYTES).uint8_memory_view()
    #    index = 7
    #    value = 42
    #    memory[index] = value

    #    self.assertEqual(memory[index], value)

    def test_get_out_of_range(self):
        with self.assertRaises(IndexError) as context_manager:
            memory = Instance(TEST_BYTES).uint8_memory_view()
            memory[len(memory) + 1]

        exception = context_manager.exception
        self.assertEqual(
            str(exception),
            'Out of bound: Absolute index 1114113 is larger than the memory size 1114112.'
        )

    #def test_set_out_of_range(self):
    #    with self.assertRaises(IndexError) as context_manager:
    #        memory = Instance(TEST_BYTES).uint8_memory_view()
    #        memory[len(memory) + 1] = 42

    #    exception = context_manager.exception
    #    self.assertEqual(
    #        str(exception),
    #        'Out of bound: Absolute index 1114113 is larger than the memory size 1114112.'
    #    )

    def test_hello_world(self):
        instance = Instance(TEST_BYTES)
        pointer = instance.exports['string']()
        memory = instance.uint8_memory_view(pointer)
        nth = 0
        string = ''

        while (0 != memory[nth]):
            string += chr(memory[nth])
            nth += 1

        self.assertEqual(string, 'Hello, World!')

    #def test_memory_views_share_the_same_buffer(self):
    #    instance = Instance(TEST_BYTES)
    #    int8 = instance.int8_memory_view()
    #    int16 = instance.int16_memory_view()
    #    int32 = instance.int32_memory_view()

    #    int8[0] = 0b00000001
    #    int8[1] = 0b00000100
    #    int8[2] = 0b00010000
    #    int8[3] = 0b01000000

    #    self.assertEqual(int8[0], 0b00000001)
    #    self.assertEqual(int8[1], 0b00000100)
    #    self.assertEqual(int8[2], 0b00010000)
    #    self.assertEqual(int8[3], 0b01000000)
    #    self.assertEqual(int16[0], 0b00000100_00000001)
    #    self.assertEqual(int16[1], 0b01000000_00010000)
    #    self.assertEqual(int32[0], 0b01000000_00010000_00000100_00000001)
