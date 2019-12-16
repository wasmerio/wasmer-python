# Compile a Rust program to Wasm.
compile-wasm FILE='examples/simple':
	#!/usr/bin/env bash
	set -euo pipefail
	rustc --target wasm32-unknown-unknown -O --crate-type=cdylib {{FILE}}.rs -o {{FILE}}.raw.wasm
	wasm-gc {{FILE}}.raw.wasm {{FILE}}.wasm
	wasm-opt -Os --strip-producers {{FILE}}.wasm -o {{FILE}}.opt.wasm
	mv {{FILE}}.opt.wasm {{FILE}}.wasm
	rm {{FILE}}.raw.wasm

# Install the environment to develop the extension.
prelude:
	#!/usr/bin/env bash
	pip3 install virtualenv
	virtualenv -p $(which python3) .env
	source .env/bin/activate
	pip3 install maturin pytest pytest-benchmark

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
	maturin develop --binding-crate pyo3 --release --strip

# Create a distribution of wasmer that can be installed
# anywhere (it will fail on import)
build-any:
	mkdir -p ./target/wheels/
	cd wasmer-any && pip3 wheel . -w ../target/wheels/

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

publish:
	maturin publish -i python3.8 python3.7 python3.6 python3.5 -u wasmer

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
