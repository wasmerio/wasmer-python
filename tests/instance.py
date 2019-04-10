from wasm import Instance, Value, validate
import inspect
import os
import unittest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()
INVALID_TEST_BYTES = open(here + '/invalid.wasm', 'rb').read()

class TestWasmInstance(unittest.TestCase):
    def test_is_a_class(self):
        self.assertTrue(inspect.isclass(Instance))

    def test_can_construct(self):
        self.assertIsInstance(Instance(TEST_BYTES), Instance)

    def test_basic_sum(self):
        self.assertEqual(
            Instance(TEST_BYTES)
                .call(
                    'sum',
                    [
                        Value.i32(1),
                        Value.i32(2)
                    ]
                ),
            3
        )

    def test_call_arity_0(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('arity_0'),
            42
        )

    def test_call_i32_i32(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('i32_i32', [Value.i32(7)]),
            7
        )

    def test_call_i64_i64(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('i64_i64', [Value.i64(7)]),
            7
        )

    def test_call_f32_f32(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('f32_f32', [Value.f32(7.)]),
            7.
        )

    def test_call_f64_f64(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('f64_f64', [Value.f64(7.)]),
            7.
        )

    def test_call_i32_i64_f32_f64_f64(self):
        self.assertEqual(
            round(
                Instance(TEST_BYTES)
                    .call(
                        'i32_i64_f32_f64_f64',
                        [
                            Value.i32(1),
                            Value.i64(2),
                            Value.f32(3.4),
                            Value.f64(5.6)
                        ]
                    ),
                6
            ),
            1 + 2 + 3.4 + 5.6
        )

    def test_call_bool_casted_to_i32(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('bool_casted_to_i32'),
            1
        )

    def test_call_string(self):
        self.assertEqual(
            Instance(TEST_BYTES).call('string'),
            1048576
        )

    def test_validate(self):
        self.assertTrue(validate(TEST_BYTES))

    def test_validate_invalid(self):
        self.assertFalse(validate(INVALID_TEST_BYTES))
