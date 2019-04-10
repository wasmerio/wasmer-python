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
result = instance.call('sum', [Value.i32(5), Value.i32(37)])

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

Finally, to inspect the extension; run:

```sh
$ just inspect
```

(Yes, you need [`just`]).

## Testing

Once the extension is compiled and installed (just run `just rust`),
run the following command:

```sh
$ just test
```

## API of the `wasm` extension/module

### The `Instance` class

Instantiates a WebAssembly module represented by bytes, and calls
exported functions on it:

```python
from wasm import Instance, Value

# Get the Wasm module as bytes.
bytes = open('my_program.wasm', 'rb').read()

# Instantiates the Wasm module.
instance = Instance(bytes)

# Call a function on it.
result = instance.call('sum', [Value.i32(1), Value.i32(2)])

print(result) # 3
```

### The `Value` class

Builds WebAssembly values with the correct types:

```python
from wasm import Value

# Integer on 32-bits.
value_i32 = Value.i32(7)

# Integer on 64-bits.
value_i64 = Value.i64(7)

# Float on 32-bits.
value_f32 = Value.f32(7.42)

# Float on 64-bits.
value_f64 = Value.f64(7.42)
```

The `Value.[if](32|64)` static methods must be considered as static
constructors.

The `to_string` method allows to get a string representation of a
`Value` instance:

```python
print(value_i32) # I32(7)
```

### The `validate` function

Checks whether the given bytes represent valid WebAssembly bytes:

```python
from wasm import validate

bytes = open('my_program.wasm', 'rb').read()

if not validate(bytes):
    print('The program seems corrupted.')
```

This function returns a boolean.

## License

The entire project is under the BSD-3-Clause license. Please read the
`LICENSE` file.


[`rust-cpython`]: https://github.com/dgrunwald/rust-cpython
[`pyo3-pack`]: https://github.com/PyO3/pyo3-pack
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
