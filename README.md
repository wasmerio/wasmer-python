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
  <a href="https://pypi.org/project/wasmer/">
      <img src="https://pepy.tech/badge/wasmer" alt="Number of downloads on Pypi" valign="middle"/>
  </a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License" valign="middle">
  </a>
</h1>

Wasmer is a Python library for executing WebAssembly binaries:

  * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
  * **Fast**: `wasmer` executes the WebAssembly modules at **native
    speed**,
  * **Safe**: All calls to WebAssembly will be fast, but more
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
from wasmer import Instance

wasm_bytes = open('simple.wasm', 'rb').read()
instance = Instance(wasm_bytes)
result = instance.exports.sum(5, 37)

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
from wasmer import Instance

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiates the Wasm module.
instance = Instance(wasm_bytes)

# Call a function on it.
result = instance.exports.sum(1, 2)

print(result) # 3
```

All exported functions are accessible on the `exports` getter.
Arguments of these functions are automatically casted to WebAssembly
values. If one wants to explicitely pass a value of a particular type,
it is possible to use the `Value` class,
e.g. `instance.exports.sum(Value.i32(1), Value.i32(2))`. Note that for
most usecases, this is not necessary.

The `memory` getter exposes the `Memory` class representing the memory
of that particular instance, e.g.:

```python
view = instance.memory.uint8_view()
```

See below for more information.

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

The `__repr__` method allows to get a string representation of a
`Value` instance:

```python
print(repr(value_i32)) # I32(7)
```

### The `Memory` class

A WebAssembly instance has its own memory, represented by the `Memory`
class. It is accessible by the `Instance.memory` getter.

The `Memory` class offers methods to create views of the memory
internal buffer, e.g. `uint8_view`, `int8_view`, `uint16_view`
etc. All these methods accept one argument: `offset`, to subset the
memory buffer at a particular offset. These methods return
respectively a `*Array` object, i.e. `uint8_view` returns a
`Uint8Array` object etc.

```python
offset = 7
view = instance.memory.uint8_view(offset)

print(view[0])
```

#### The `*Array` classes

These classes represent views over a memory buffer of an instance.

| Class | View buffer as a sequence of… | Bytes per element |
|-|-|-|
| `Int8Array` | `int8` | 1 |
| `Uint8Array` | `uint8` | 1 |
| `Int16Array` | `int16` | 2 |
| `Uint16Array` | `uint16` | 2 |
| `Int32Array` | `int32` | 4 |
| `Uint32Array` | `uint32` | 4 |

All these classes share the same implementation. Taking the example of
`Uint8Array`, the class looks like this:

```python
class Uint8Array:
    @property
    def bytes_per_element()

    def __len__()
    def __getitem__(index|slice)
    def __setitem__(index, value)
```

Let's see it in action:

``` python
from wasmer import Instance

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiates the Wasm module.
instance = Instance(wasm_bytes)

# Call a function that returns a pointer to a string for instance.
pointer = instance.exports.return_string()

# Get the memory view, with the offset set to `pointer` (default is 0).
memory = instance.memory.uint8_view(pointer)

# Read the string pointed by the pointer.
nth = 0;
string = ''

while True:
    char = memory[nth]
    
    if char == 0:
        break

    string += chr(char)
    nth += 1

print(string) # Hello, World!
```

A slice can be used as index of the `__getitem__` method, which is
useful when we already know the size of the data we want to read, e.g.:

```python
print(''.join(map(chr, memory[0:13]))) # Hello, World!
```

Notice that `*Array` treat bytes in little-endian, as required by the
WebAssembly specification, [Chapter Structure, Section Instructions,
Sub-Section Memory
Instructions](https://webassembly.github.io/spec/core/syntax/instructions.html#memory-instructions):

> All values are read and written in [little
> endian](https://en.wikipedia.org/wiki/Endianness#Little-endian) byte
> order.

Each view shares the same memory buffer internally. Let's have some fun:

``` python
int8 = instance.memory.int8_view()
int16 = instance.memory.int16_view()
int32 = instance.memory.int32_view()

               b₁
            ┌┬┬┬┬┬┬┐
int8[0] = 0b00000001
               b₂
            ┌┬┬┬┬┬┬┐
int8[1] = 0b00000100
               b₃
            ┌┬┬┬┬┬┬┐
int8[2] = 0b00010000
               b₄
            ┌┬┬┬┬┬┬┐
int8[3] = 0b01000000

// No surprise with the following assertions.
                       b₁
                    ┌┬┬┬┬┬┬┐
assert int8[0] == 0b00000001
                       b₂
                    ┌┬┬┬┬┬┬┐
assert int8[1] == 0b00000100
                       b₃
                    ┌┬┬┬┬┬┬┐
assert int8[2] == 0b00010000
                       b₄
                    ┌┬┬┬┬┬┬┐
assert int8[3] == 0b01000000

// The `int16` view reads 2 bytes.
                        b₂       b₁
                     ┌┬┬┬┬┬┬┐ ┌┬┬┬┬┬┬┐
assert int16[0] == 0b00000100_00000001
                        b₄       b₃
                     ┌┬┬┬┬┬┬┐ ┌┬┬┬┬┬┬┐
assert int16[1] == 0b01000000_00010000

// The `int32` view reads 4 bytes.
                        b₄       b₃       b₂       b₁
                     ┌┬┬┬┬┬┬┐ ┌┬┬┬┬┬┬┐ ┌┬┬┬┬┬┬┐ ┌┬┬┬┬┬┬┐
assert int32[0] == 0b01000000_00010000_00000100_00000001;
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

The entire project is under the BSD-3-Clause license. Please read [the
`LICENSE` file][license].


[Pypi]: https://pypi.org/
[`rust-cpython`]: https://github.com/dgrunwald/rust-cpython
[`pyo3-pack`]: https://github.com/PyO3/pyo3-pack
[`virtualenv`]: https://virtualenv.pypa.io/
[`just`]: https://github.com/casey/just/
[license]: https://github.com/wasmerio/wasmer/blob/master/LICENSE
