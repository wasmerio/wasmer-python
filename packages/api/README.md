# <img height="48" src="https://wasmer.io/static/icons/favicon-96x96.png" alt="Wasmer logo" valign="middle"> Wasmer Python [![PyPI version](https://img.shields.io/pypi/v/wasmer)](https://badge.fury.io/py/wasmer) [![Wasmer Python Documentation](https://img.shields.io/badge/docs-read-green)](https://wasmerio.github.io/wasmer-python/api/) [![Wasmer PyPI downloads](https://pepy.tech/badge/wasmer)](https://pypi.org/project/wasmer/) [![Wasmer Slack Channel](https://img.shields.io/static/v1?label=chat&message=on%20Slack&color=green)](https://slack.wasmer.io)

A complete and mature WebAssembly runtime for Python based on
[Wasmer](https://github.com/wasmerio/wasmer).

Features:

  * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
  * **Fast**: `wasmer` executes the WebAssembly modules as fast as
    possible, close to **native speed**,
  * **Safe**: All calls to WebAssembly will be fast, but more
    importantly, completely safe and sandboxed,
  * **Modular**: `wasmer` can compile the WebAssembly modules with
    different engines or compiler.

**Documentation**: [browse the detailed API
documentation](https://wasmerio.github.io/wasmer-python/api/wasmer/) full of
examples.

**Examples** as tutorials: [browse the `examples/`
directory](https://github.com/wasmerio/wasmer-python/tree/master/examples),
it's the best place for a complete introduction!

## Quick Introduction

The `wasmer` package brings the required API to execute WebAssembly
modules. In a nutshell, `wasmer` compiles the WebAssembly module into
compiled code, and then executes it. `wasmer` is designed to work in
various environments and platforms: From nano single-board computers
to large and powerful servers, including more exotic ones. To address
those requirements, Wasmer provides 2 engines and 3 compilers.

Succinctly, an _engine_ is responsible to drive the _compilation_ and
the _execution_ of a WebAssembly module. By extension, a _headless_
engine can only execute a WebAssembly module, i.e. a module that has
previously been compiled, or compiled, serialized and deserialized. By
default, the `wasmer` package comes with 2 headless engines:

1. `wasmer.engine.JIT`, the compiled machine code lives in memory,
2. `wasmer.engine.Native`, the compiled machine code lives in a shared
   object file (`.so`, `.dylib`, or `.dll`), and is natively executed.

Because `wasmer` does not embed compilers in its package, engines are
headless, i.e. they can't compile WebAssembly module; they can only
execute them. Compilers live in their own standalone packages. Let's
briefly introduce them:

| Compiler package | Description | PyPi |
|-|-|-|
| `wasmer_compiler_singlepass` | Super fast compilation times, slower execution times. Not prone to JIT-bombs. *Ideal for blockchains* | [![On PyPi](https://img.shields.io/pypi/v/wasmer_compiler_singlepass)](https://pypi.org/project/wasmer_compiler_singlepass/) |
| `wasmer_compiler_cranelift` | Fast compilation times, fast execution times. *Ideal for development* | [![On PyPi](https://img.shields.io/pypi/v/wasmer_compiler_cranelift)](https://pypi.org/project/wasmer_compiler_cranelift/) |
| `wasmer_compiler_llvm` | Slow compilation times, very fast execution times (close to native, sometimes faster). *Ideal for Production* | [![On PyPi](https://img.shields.io/pypi/v/wasmer_compiler_llvm)](https://pypi.org/project/wasmer_compiler_llvm/) |

We generally recommend `wasmer_compiler_cranelift` for development
purposes and `wasmer_compiler_llvm` in production.

Learn more by reading [the documentation of the `wasmer.engine`
submodule](https://wasmerio.github.io/wasmer-python/api/engine/index.html).

## Install

To install the `wasmer` Python package, and let's say the
`wasmer_compiler_cranelift` compiler, just run those commands in your shell:

```sh
$ pip install wasmer
$ pip install wasmer_compiler_cranelift
```

And you're ready to get fun!

## Example

We highly recommend to the read the
[`examples/`](https://github.com/wasmerio/wasmer-python/tree/master/examples)
directory, which contains a sequence of examples/tutorials. It's the
best place to learn by reading examples.

But for the most eager of you, and we know you're numerous you
mischievous, there is a quick toy program in
`examples/appendices/simple.rs`, written in Rust:

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

After compilation to WebAssembly, the
[`examples/appendices/simple.wasm`](https://github.com/wasmerio/wasmer-python/blob/master/examples/appendices/simple.wasm)
binary file is generated. ([Download
it](https://github.com/wasmerio/wasmer-python/raw/master/examples/appendices/simple.wasm)).

Then, we can excecute it in Python:

```python
from wasmer import engine, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

# Let's define the store, that holds the engine, that holds the compiler.
store = Store(engine.JIT(Compiler))

# Let's compile the module to be able to execute it!
module = Module(store, open('simple.wasm', 'rb').read())

# Now the module is compiled, we can instantiate it.
instance = Instance(module)

# Call the exported `sum` function.
result = instance.exports.sum(5, 37)

print(result) # 42!
```

And then, finally, enjoy by running:

```sh
$ python examples/appendices/simple.py
```

# Development

The Python extension is written in [Rust], with [`pyo3`] and
[`maturin`].

First, you need to install Rust and Python. We will not make you the
affront to explain to you how to install Python (if you really need,
check [`pyenv`](https://github.com/pyenv/pyenv/)). For Rust though, we
advise to use [`rustup`](https://rustup.rs/), then:

```sh
$ rustup install stable
```

To set up your environment, you'll need [`just`], and then, install
the prelude of this project:

```sh
$ cargo install just
$ just --list # to learn about all the available recipes
$ just prelude
```

It will install `pyo3` and `maturin` for Python and for Rust. It will
also install [`virtualenv`].

Then, simply run:

```sh
$ source .env/bin/activate
$ just build api
$ just build compiler-cranelift
$ just python-run examples/appendices/simple.py
```

If you need to interact with Python, or run a specific file, use the
following commands:

```sh
$ just python-run
$ just python-run file/to/run.py
```

## Supported platforms

We try to provide wheels for as many platforms and architectures as
possible. For the moment, here are the supported platforms and
architectures:

| Platform | Architecture | Triple |
|-|-|-|
| Linux | `amd64` | `x86_64-unknown-linux-gnu` |
| Linux | `aarch64` | `aarch64-unknown-linux-gnu` |
| Darwin | `amd64` | `x86_64-apple-darwin` |
| Windows | `amd64` | `x86_64-pc-windows-msvc` |

Wheels are all built for the following Python versions:

* Python 3.5,
* Python 3.6,
* Python 3.7,
* Python 3.8.

<details>
<summary>Learn about the “fallback” <code>py3-none-any</code> wheel</summary>

### `py3-none-any.whl`

A special `wasmer-$(version)-py3-none-any` wheel is built as a
fallback. The `wasmer` libray will be installable, but it will raise
an `ImportError` exception saying that “Wasmer is not available on
this system”.

This wheel will be installed if none matches before (learn more by
reading the [PEP 425, Compatibility Tags for Built
Distributions](https://www.python.org/dev/peps/pep-0425/)).

</details>

## Testing

Build all the packages and run the tests:

```sh
$ just build-all
$ just test
```

# What is WebAssembly?

Quoting [the WebAssembly site](https://webassembly.org/):

> WebAssembly (abbreviated Wasm) is a binary instruction format for a
> stack-based virtual machine. Wasm is designed as a portable target
> for compilation of high-level languages like C/C++/Rust, enabling
> deployment on the web for client and server applications.

About speed:

> WebAssembly aims to execute at native speed by taking advantage of
> [common hardware
> capabilities](https://webassembly.org/docs/portability/#assumptions-for-efficient-execution)
> available on a wide range of platforms.

About safety:

> WebAssembly describes a memory-safe, sandboxed [execution
> environment](https://webassembly.org/docs/semantics/#linear-memory) […].

# License

The entire project is under the MIT License. Please read [the
`LICENSE` file][license].


[`pyo3`]: https://github.com/PyO3/pyo3
[`maturin`]: https://github.com/PyO3/maturin
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
[license]: https://github.com/wasmerio/wasmer/blob/master/LICENSE
[Rust]: https://www.rust-lang.org/
[compilers]: https://medium.com/wasmer/a-webassembly-compiler-tale-9ef37aa3b537
