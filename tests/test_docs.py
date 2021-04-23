import pytest

import wasmer

known_issues = {
    wasmer.Table.type,
    wasmer.engine,
    wasmer.target,
    wasmer.wasi.Environment.generate_import_object,
    wasmer.wasi.StateBuilder.preopen_directories,
    wasmer.wasi.StateBuilder.preopen_directory,
}

def test_docexample(docexample):
    if docexample.obj in known_issues:
        pytest.xfail()

    docexample.exec()
