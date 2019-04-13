# Install the environment to develop the extension.
prelude:
        virtualenv -p $(which python3) .env
        source .env/bin/activate

        pip3 install pyo3-pack pytest pytest-benchmark
        pip3 install virtualenv

# Setup the environment to develop the extension.
wakeup:
        source .env/bin/activate

# Unset the development environment.
sleep:
        deactivate

# Compile and install the Rust library.
rust: wakeup
        export PYTHON_SYS_EXECUTABLE=$(which python3)
        cargo check
        pyo3-pack develop --binding_crate pyo3 --release --strip

# Run Python.
python-run file='': wakeup
        python {{file}}

# Run the tests.
test: wakeup
        py.test tests

# Inspect the `python-ext-wasm` extension.
inspect: wakeup
	python -c "help('wasmer')"

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
