import instance
import memory_view
import unittest
import value

def add_tests_from(suite, test_case):
    suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(test_case))

def suite():
    suite = unittest.TestSuite()
    add_tests_from(suite, instance.TestWasmInstance)
    add_tests_from(suite, memory_view.TestWasmMemoryView)
    add_tests_from(suite, value.TestWasmValue)
    return suite

if __name__ == '__main__':
    runner = unittest.TextTestRunner(verbosity=2);
    runner.run(suite())
