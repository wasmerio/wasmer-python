from wasmer import Instance, Uint8MemoryView, Value, validate
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

    def test_failed_to_instantiate(self):
        with self.assertRaises(RuntimeError) as context_manager:
            Instance(INVALID_TEST_BYTES)

        exception = context_manager.exception
        self.assertEqual(
            str(exception),
            'Failed to instantiate the module:\n    compile error: Validation error "Invalid type"'
        )

    def test_function_does_not_exist(self):
        with self.assertRaises(KeyError) as context_manager:
            Instance(TEST_BYTES).exports['foo']

        exception = context_manager.exception
        self.assertEqual(
            str(exception),
            "'foo'"
        )

    def test_basic_sum(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['sum'](1, 2),
            3
        )

    def test_call_arity_0(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['arity_0'](),
            42
        )

    def test_call_i32_i32(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['i32_i32'](7),
            7
        )

    def test_call_i64_i64(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['i64_i64'](7),
            7
        )

    def test_call_f32_f32(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['f32_f32'](7.),
            7.
        )

    def test_call_f64_f64(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['f64_f64'](7.),
            7.
        )

    def test_call_i32_i64_f32_f64_f64(self):
        self.assertEqual(
            round(Instance(TEST_BYTES).exports['i32_i64_f32_f64_f64'](1, 2, 3.4, 5.6), 6),
            1 + 2 + 3.4 + 5.6
        )

    def test_call_bool_casted_to_i32(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['bool_casted_to_i32'](),
            1
        )

    def test_call_string(self):
        self.assertEqual(
            Instance(TEST_BYTES).exports['string'](),
            1048576
        )

    def test_validate(self):
        self.assertTrue(validate(TEST_BYTES))

    def test_validate_invalid(self):
        self.assertFalse(validate(INVALID_TEST_BYTES))

    def test_memory_view(self):
        self.assertIsInstance(Instance(TEST_BYTES).uint8_memory_view(), Uint8MemoryView)
