# Install the environment to develop the extension.
prelude:
	#!/usr/bin/env bash
	pip3 install virtualenv
	virtualenv -p $(which python3) .env
	source .env/bin/activate
	pip3 install pyo3-pack pytest pytest-benchmark

# Setup the environment to develop the extension.
wakeup:
	#!/usr/bin/env bash
	source .env/bin/activate

# Unset the development environment.
sleep:
	deactivate

# Compile and install the Python library.
build:
	export PYTHON_SYS_EXECUTABLE=$(which python3)
	cargo check
	pyo3-pack develop --binding_crate pyo3 --release --strip

# Run Python.
python-run file='':
	@python {{file}}

# Run the tests.
test:
	@py.test -v tests

# Run one or more benchmarks.
benchmark benchmark-filename='':
	@py.test benchmarks/{{benchmark-filename}}

# Inspect the `python-ext-wasm` extension.
inspect:
	@python -c "help('wasmer')"

# Create a distribution of wasmer that can be installed
# anywhere (it will fail on import)
wasmer_any:
	mkdir -p ./target/wheels/
	cd wasmer-any && pip3 wheel . -w ../target/wheels/

publish:
	pyo3-pack publish -i python3.7 python3.6 python3.5 -u wasmer

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
