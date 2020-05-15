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
	set -x

	pip3 install virtualenv
	virtualenv .env
	if test -d .env/bin/; then source .env/bin/activate; else source .env/Scripts/activate; fi
	pip3 install maturin pytest pytest-benchmark

	which maturin
	maturin --version
	which python
	python --version
	which python-config
	python-config --abiflags || true
	pwd
	ls -l .env

# Setup the environment to develop the extension.
wakeup:
	#!/usr/bin/env bash
	if test -d .env/bin/; then source .env/bin/activate; else source .env/Scripts/activate; fi

# Unset the development environment.
sleep:
	deactivate

build_features := ""

# Compile and install the Python library.
# Run with `--set build_features` to compile with specific Cargo features.
build:
        #!/usr/bin/env bash
        export PYTHON_SYS_EXECUTABLE=$(which python)

        build_features="{{build_features}}"

        if test -z "${build_features}"; then
                if test "{{arch()}}" = "aarch64"; then
                        build_features="backend-singlepass";
                fi
        fi

        build_args=""

        if test ! -z "${build_features}"; then
                build_args="--no-default-features --features ${build_features}"
        fi

        echo "Build arguments: ${build_args}"

        cargo check ${build_args}
        maturin develop --binding-crate pyo3 --release --strip --cargo-extra-args="${build_args}"

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

publish version:
	maturin publish -i {{version}} -u wasmer

publish-any:
	twine upload --repository-url https://upload.pypi.org/legacy/ target/wheels/wasmer-*-py3-none-any.whl -u wasmer

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
