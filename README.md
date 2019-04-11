<h1>
  <a href="https://wasmer.io" target="_blank" rel="noopener noreferrer" valign="middle">
    <img height="64" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/logo.png" alt="Wasmer logo" valign="middle">
  </a>
  &nbsp;
  <a href="https://spectrum.chat/wasmer">
    <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community" valign="middle">
  </a>
  <a href="https://pypi.org/project/wasmer/">
      <img src="https://img.shields.io/pypi/format/wasmer.svg" alt="Pypi" valign="middle"/>
  </a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License" valign="middle">
  </a>
</h1>

Wasmer is a Python library for executing WebAssembly binaries:

  * **Easy to use:** wasmer API mimics the standard WebAssembly API,
  * **Fast:** wasmer executes the WebAssembly modules at **native
    speed**,
  * **Safe:** all calls to WebAssembly will be fast, but more
    importantly, completely safe and sandboxed.

## Install

To install the `wasmer` Python library, just run this command in your
shell:

```sh
$ pip install wasmer
```

**Note**: There is a limited set of wheels published so far. More are
coming.

[View the `wasmer` on Pypi](https://pypi.org/project/wasmer/).

## Example

There is a toy program in `examples/simple.rs`, written in Rust (or
any other language that compiles to Wasm):

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

After compilation to Wasm, we obtain the
[`examples/simple.wasm`](https://github.com/wasmerio/python-ext-wasm/blob/master/examples/simple.wasm)
binary file. ([Download
it](https://github.com/wasmerio/python-ext-wasm/blob/master/examples/simple.wasm)).

Then, we can excecute it in Python:

```python
from wasmer import Instance, Value

wasm_bytes = open('simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
result = instance.call('sum', [Value.i32(5), Value.i32(37)])

print(result) # 42!
```

And then, finally, enjoy by running:

```sh
$ python examples/simple.py
```

## API of the `wasm` extension/module

### The `Instance` class

Instantiates a WebAssembly module represented by bytes, and calls
exported functions on it:

```python
from wasmer import Instance, Value

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiates the Wasm module.
instance = Instance(wasm_bytes)

# Call a function on it.
result = instance.call('sum', [Value.i32(1), Value.i32(2)])

print(result) # 3
```

### The `Value` class

Builds WebAssembly values with the correct types:

```python
from wasmer import Value

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

### The `MemoryView` class

Represents a view over a memory buffer of an instance:

``` python
from wasmer import Instance

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiates the Wasm module.
instance = Instance(wasm_bytes)

# Call a function that returns a pointer to a string for instance.
pointer = instance.call('return_string')

# Get the memory view, with the offset set to `pointer` (default is 0).
memory = instance.memory_view(pointer)

# Read the string pointed by the pointer.
nth = 0;
string = ''

while (0 != memory.get(nth)):
    string += chr(memory.get(nth))
    nth += 1

print(string) # Hello, World!
```

### The `validate` function

Checks whether the given bytes represent valid WebAssembly bytes:

```python
from wasmer import validate

wasm_bytes = open('my_program.wasm', 'rb').read()

if not validate(wasm_bytes):
    print('The program seems corrupted.')
```

This function returns a boolean.

## Development

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

## License

The entire project is under the BSD-3-Clause license. Please read the
`LICENSE` file.


[Pypi]: https://pypi.org/
[`rust-cpython`]: https://github.com/dgrunwald/rust-cpython
[`pyo3-pack`]: https://github.com/PyO3/pyo3-pack
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
