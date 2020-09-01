<p align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener">
    <img width="300" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/assets/logo.png" alt="Wasmer logo">
  </a>
</p>

<p align="center">
  <a href="https://pypi.org/project/wasmer/">
      <img src="https://pepy.tech/badge/wasmer" alt="Number of downloads on Pypi" valign="middle"/></a>
  <a href="https://slack.wasmer.io">
    <img src="https://img.shields.io/static/v1?label=Slack&message=join%20chat&color=brighgreen&style=flat-square" alt="Slack channel">
  </a> 
</p>

[Wasmer] is an advanced and mature WebAssembly runtime. The `wasmer`
Python package is a native Python extension to embed Wasmer inside
Python. Wasmer is:

  * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
  * **Fast**: `wasmer` executes the WebAssembly modules as fast as
    possible, close to **native speed**,
  * **Safe**: All calls to WebAssembly will be fast, but more
    importantly, completely safe and sandboxed.

[Wasmer]: https://github.com/wasmerio/wasmer

# Install

To install the `wasmer` Python library, just run this command in your
shell:

```sh
$ pip install wasmer
```

**Note**: There is a limited set of wheels published so far. More are
coming.

[View the `wasmer` on Pypi](https://pypi.org/project/wasmer/).

## Wheels

We try to provide wheels for as many platforms and architectures as
possible. [Wasmer, the runtime](https://github.com/wasmerio/wasmer),
provides several compiler backends, which address different needs and
contexts ([learn more][compiler-backends]). While it is possible to
force one compiler backend for your own setup, the wheels come
pre-packaged with a particular one. For the moment, here are the
supported platforms and architectures:

| Platform | Architecture | Triple | Default compiler backend |
|-|-|-|-|
| Linux | `amd64` | `x86_64-unknown-linux-gnu` | Cranelift |
| Linux | `aarch64` | `aarch64-unknown-linux-gnu` | Singlepass |
| Darwin | `amd64` | `x86_64-apple-darwin` | Cranelift |
| Windows | `amd64` | `x86_64-pc-windows-msvc` | Cranelift |

Note: it's also possible to [build Wasmer in Python with a specific
backend](#use-a-particular-wasmer-compiler-backend), for example using
LLVM for extra speed.

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

# Example

There is a toy program in `examples/simple.rs`, written in Rust (or
any other language that compiles to WebAssembly):

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

After compilation to WebAssembly, the
[`examples/simple.wasm`](https://github.com/wasmerio/python-ext-wasm/blob/master/examples/simple.wasm)
binary file is generated. ([Download
it](https://github.com/wasmerio/python-ext-wasm/raw/master/examples/simple.wasm)).

Then, we can excecute it in Python:

```python
from wasmer import Store, Module, Instance

store = Store()
module = Module(store, open('simple.wasm', 'rb').read())
instance = Instance(module)
result = instance.exports.sum(5, 37)

print(result) # 42!
```

And then, finally, enjoy by running:

```sh
$ python examples/simple.py
```

# API of the `wasmer` package

Browse the documentation at
https://wasmerio.github.io/python-ext-wasm/api/.

Alternatively, run `just build` followed by `just doc` to generate the
documentation inside `docs/api/index.html`.

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
$ just prelude
```

It will install `pyo3` and `maturin` for Python and for Rust. It will
also install [`virtualenv`].

Then, simply run:

```sh
$ source .env/bin/activate
$ just build
$ just python-run examples/simple.py
```

If you need to interact with Python, or run a specific file, use the
following commands:

```sh
$ just python-run
$ just python-run file/to/run.py
```

Finally, to inspect the extension; run:

```sh
$ just inspect
```

## Use a particular Wasmer compiler backend

Wasmer, the runtime, comes with several compiler backends addressing
particular needs or contexts ([learn more][compiler-backends]). To set
the compiler backend to use, the `Cargo.toml` file exposes 3 features:

* `backend-singlepass`,
* `backend-cranelift` and
* `backend-llvm`.

To enable those features with `just build`, use such syntax:

```sh
$ just --set build_features backend-llvm build
```

# Testing

Once the extension is compiled and installed (with `just build`), run
the following command:

```sh
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


[Pypi]: https://pypi.org/
[`rust-cpython`]: https://github.com/dgrunwald/rust-cpython
[`pyo3`]: https://github.com/PyO3/pyo3
[`maturin`]: https://github.com/PyO3/maturin
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
[license]: https://github.com/wasmerio/wasmer/blob/master/LICENSE
[Rust]: https://www.rust-lang.org/
[compiler-backends]: https://medium.com/wasmer/a-webassembly-compiler-tale-9ef37aa3b537
