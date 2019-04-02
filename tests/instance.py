from wasm import Instance, Value
import inspect
import os
import unittest

here = os.path.dirname(os.path.realpath(__file__))
file = open(here + '/tests.wasm', 'rb')

TEST_BYTES = file.read()

class TestWasmInstance(unittest.TestCase):
    def test_is_a_class(self):
        self.assertTrue(inspect.isclass(Instance))

    def test_can_construct(self):
        self.assertTrue(type(Instance(TEST_BYTES)) is Instance)

    def test_basic_sum(self):
        self.assertEqual(
            Instance(TEST_BYTES)
                .call(
                    'sum',
                    [
                        Value.from_i32(1),
                        Value.from_i32(2)
                    ]
                ),
            3
        )

    def test_arity_0(self):
        self.assertEqual(
            Instance(TEST_BYTES)
                .call('arity_0', []),
            42
        )
