# Install the environment to develop the extension.
prelude:
        pip3 install virtualenv
        virtualenv -p $(which python3) .env
        source .env/bin/activate
        pip3 install pyo3-pack pytest pytest-benchmark

# Setup the environment to develop the extension.
wakeup:
        source .env/bin/activate

# Unset the development environment.
sleep:
        deactivate

# Compile and install the Rust library.
rust:
        export PYTHON_SYS_EXECUTABLE=$(which python3)
        cargo check
        pyo3-pack develop --binding_crate pyo3 --release --strip

# Run Python.
python-run file='':
        @python {{file}}

# Run the tests.
test:
        @py.test tests --benchmark-skip

# Benchmark the tests.
bench:
        @py.test tests --benchmark-only

# Inspect the `python-ext-wasm` extension.
inspect:
	@python -c "help('wasmer')"

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
