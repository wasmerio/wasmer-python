from wasmer import Instance
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/imported_function.wasm', 'rb').read()

#def sum(x: int, y: int) -> int:
#    return x + y
#
#def test_imported_function():
#    instance = Instance(
#        TEST_BYTES,
#        {
#            "env": {
#                "sum": sum
#            }
#        }
#    )
#
#    assert instance.exports.add_one(1, 2) == 4
