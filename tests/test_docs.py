import pytest

import wasmer

known_issues = {
    wasmer.wasi.StateBuilder.preopen_directories,
    wasmer.wasi.StateBuilder.preopen_directory,
}

def test_doctest(doctest):
    if doctest.obj in known_issues:
        pytest.xfail()

    doctest.exec()
