import unittest
import value
import instance

def add_tests_from(suite, test_case):
    suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(test_case))

def suite():
    suite = unittest.TestSuite()
    add_tests_from(suite, value.TestWasmValue)
    add_tests_from(suite, instance.TestWasmInstance)
    return suite

if __name__ == '__main__':
    runner = unittest.TextTestRunner(verbosity=2);
    runner.run(suite())
