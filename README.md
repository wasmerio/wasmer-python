# `python-ext-wasm`

## Installation

```sh
$ export PYTHON_SYS_EXECUTABLE=$(which python3)
$ pip3 install pyo3-pack
$ cargo install pyo3-pack
$ pip3 install --user virtualenv
$ virtualenv .env
$ source .env/bin/activate

$ cargo build
$ pyo3-pack develop
$ ./env/bin/python
>>> import wasm
```
