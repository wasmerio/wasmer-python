from wasm import Value
import inspect
import unittest

class TestWasmValue(unittest.TestCase):
    def test_is_a_class(self):
        self.assertTrue(inspect.isclass(Value))

    @unittest.expectedFailure
    def test_cannot_construct(self):
        Value()

    def test_i32(self):
        self.assertEqual(Value.from_i32(42).to_string(), 'I32(42)')

    def test_i64(self):
        self.assertEqual(Value.from_i64(42).to_string(), 'I64(42)')

    def test_f32(self):
        self.assertEqual(Value.from_f32(4.2).to_string(), 'F32(4.2)')

    def test_f32_auto_cast(self):
        self.assertEqual(Value.from_f32(42).to_string(), 'F32(42.0)')

    def test_f64(self):
        self.assertEqual(Value.from_f64(4.2).to_string(), 'F64(4.2)')

    def test_f64_auto_cast(self):
        self.assertEqual(Value.from_f64(42).to_string(), 'F64(42.0)')
