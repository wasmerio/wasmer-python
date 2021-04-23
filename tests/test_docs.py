import pytest

import wasmer

known_issues = []

def test_doctest(doctest):
    if doctest.obj in known_issues:
        pytest.xfail()

    doctest.exec()
