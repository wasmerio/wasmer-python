import pytest

import wasmer

known_issues =  {
    # wasmer_compiler_llvm is missing on Windows
    wasmer.engine,
    wasmer.Store,
}

def test_doctest(doctest):
    if doctest.obj in known_issues:
        pytest.xfail()

    doctest.exec()
