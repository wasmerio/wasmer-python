# 🐍+🦀+🕸 = Python `ext-wasm`

_This is only experimental right now_.

The goal of the project is to be able to run WebAssembly binary from
Python directly.

## What is WebAssembly?

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

## Goals

This extension has some goals in minds. Let's list some of them:

_[under writing]_

## Example

There is a toy program in `examples/simple.rs`, written in Rust (or
any other language that compiles to Wasm):

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

After compilation to Wasm, we end up with a `examples/simple.wasm`
binary file.

Then, we can excecute it in Python (!) with the `examples/simple.py`
file:

```python
from wasm import Instance, Value

bytes = open('simple.wasm', 'rb').read()
instance = Instance(bytes)
result = instance.call('sum', [Value.from_i32(5), Value.from_i32(37)])

print(result) # 42!
```

And then, finally, enjoy by running:

```sh
$ .env/bin/python examples/simple.py
```

## Usage

The Python extension is written in Rust, with [`rust-cpython`] and
[`pyo3-pack`].

To set up your environment, run only once:

```sh
$ just prelude
```

It will install `pyo3-pack` for Python and for Rust. It will also
install [`virtualenv`].

Then, simply run:

```sh
$ .env/bin/activate
$ just rust
$ just python-run examples/simple.py
```

If you need to interact with Python, or run a specific file, use the
following commands:

```sh
$ just python-run
$ just python-run file/to/run.py
```

Finally, to get help about the example, run:

```sh
$ just help
```

You are likely to see something like this:

```
Help on module libwasm:

NAME
    libwasm - This extension exposes an API to manipulate and to execute WebAssembly binaries.

CLASSES
    builtins.object
        Instance
        Value

    class Instance(builtins.object)
     |  Methods defined here:
     |
     |  call(...)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Value(builtins.object)
     |  Methods defined here:
     |
     |  to_string(...)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  from_f32(...)
     |
     |  from_f64(...)
     |
     |  from_i32(...)
     |
     |  from_i64(...)
```

(yes, you need [`just`]).

## Testing

Once the extension is compiled and installed (just run `just rust`),
run the following command:

```sh
$ just test
```

## License

The entire project is under the BSD-3-Clause license. Please read the
`LICENSE` file.


[`rust-cpython`]: https://github.com/dgrunwald/rust-cpython
[`pyo3-pack`]: https://github.com/PyO3/pyo3-pack
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
