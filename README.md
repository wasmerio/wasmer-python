<p align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener">
    <img width="300" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/assets/logo.png" alt="Wasmer logo">
  </a>
</p>

<p align="center">
  <a href="https://spectrum.chat/wasmer">
    <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community" valign="middle"></a>
  <a href="https://pypi.org/project/wasmer/">
      <img src="https://img.shields.io/pypi/format/wasmer.svg" alt="Pypi" valign="middle"/></a>
  <a href="https://pypi.org/project/wasmer/">
      <img src="https://pepy.tech/badge/wasmer" alt="Number of downloads on Pypi" valign="middle"/></a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License" valign="middle"></a>
</p>

Wasmer is a Python library for executing WebAssembly binaries:

  * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
  * **Fast**: `wasmer` executes the WebAssembly modules as fast as
    possible, close to **native speed**,
  * **Safe**: All calls to WebAssembly will be fast, but more
    importantly, completely safe and sandboxed.

# Install

To install the `wasmer` Python library, just run this command in your
shell:

```sh
$ pip install wasmer
```

**Note**: There is a limited set of wheels published so far. More are
coming.

[View the `wasmer` on Pypi](https://pypi.org/project/wasmer/).

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

For a soft introduction about how different languages compile to Wasm, it is possible to give it a try at [WebAssembly Studio](https://webassembly.studio).

# API of the `wasmer` extension/module

## The `Instance` class

Instantiates a WebAssembly module represented by bytes, and calls
exported functions on it:

```python
from wasmer import Instance

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiate the Wasm module.
instance = Instance(wasm_bytes)

# Call a function on it.
result = instance.exports.sum(1, 2)

print(result) # 3
```

### Exported functions

All exported functions are accessible on the `exports` getter.
Arguments of these functions are automatically casted to WebAssembly
values. If one wants to explicitely pass a value of a particular type,
it is possible to use the `Value` class,
e.g. `instance.exports.sum(Value.i32(1), Value.i32(2))`. Note that for
most usecases, this is not necessary.

### Exported memory

The `memory` getter exposes the `Memory` class representing the memory
of that particular instance, e.g.:

```python
view = instance.memory.uint8_view()
```

`Instance.memory` can return `None` if no memory is exported.

See below for more information.

## The `Module` class

Compiles a sequence of bytes into a WebAssembly module. From here, it
is possible to instantiate it:

```python
from wasmer import Module

# Get the Wasm bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Compile the bytes into a Wasm module.
module = Module(wasm_bytes)

# Instantiate the Wasm module.
instance = module.instantiate()

# Call a function on it.
result = instance.exports.sum(1, 2)

print(result) # 3
```

### Exports, imports, and custom sections

It is also possible to query the module to get a list of exports, of
imports, or of custom sections.

```python
from wasmer import Module, ExportKind, ImportKind

# Get the Wasm bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Compile the bytes into a Wasm module.
module = Module(wasm_bytes)

# Check all the exports.
assert module.exports == [{'kind': ExportKind.MEMORY,   'name': 'memory'},
                          {'kind': ExportKind.TABLE,    'name': '__indirect_function_table'},
                          {'kind': ExportKind.GLOBAL,   'name': '__heap_base'},
                          {'kind': ExportKind.GLOBAL,   'name': '__data_end'},
                          {'kind': ExportKind.FUNCTION, 'name': 'sum'}]

# Check all the imports.
assert module.imports == [{'kind': ImportKind.FUNCTION,
                           'namespace': 'ns',
                           'name': 'func'},
                          {'kind': ImportKind.MEMORY,
                           'namespace': 'ns',
                           'name': 'mem',
                           'minimum_pages': 3,
                           'maximum_pages': 4},
                          {'kind': ImportKind.GLOBAL,
                           'namespace': 'ns',
                           'name': 'glo',
                           'mutable': False,
                           'type': 'f32'},
                          {'kind': ImportKind.TABLE,
                           'namespace': 'ns',
                           'name': 'tab',
                           'minimum_elements': 1,
                           'maximum_elements': 2,
                           'element_type': 'anyfunc'}]

# Check all the custom sections.
assert sorted(module.custom_section_names) == ['section1', 'section2']

# Check one specific custom section content (in bytes).
custom_section1 = module.custom_section('section1')

assert type(custom_section1) == bytes
assert custom_sections1 == b'Wasmer'
```

Note the `ExportKind` and `ImportKind` enumerations. They are precisely
[`IntEnum`](https://docs.python.org/3/library/enum.html#enum.IntEnum).

`Module.exports` always returns a list of dictionaries with the
`kind` and `name` pairs. `Module.imports` always returns a list of
dictionaries with at least the `namespace` and `name` pairs. Some
specific pairs exist, see the following table.

| `ExportKind`/`ImportKind` variants | Meaning | Specific pairs for `imports` |
|-|-|-|
| `FUNCTION` | Function | none |
| `GLOBAL` | Global variable | `mutable` and `type` |
| `MEMORY` | Memory | `minimum_pages` and `maximum_pages` (`None` if absent) |
| `TABLE` | Table | `minimum_elements`, `maximum_elements` (`None` is absent) and `element_type` |

### Serialization and deserialization

The `Module.serialize` method and its complementary
`Module.deserialize` static method help to respectively serialize and
deserialize a compiled WebAssembly module, thus saving the compilation
time for the next use:

```python
from wasmer import Module

# Get the Wasm bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Compile the bytes into a Wasm module.
module1 = Module(wasm_bytes)

# Serialize the module.
serialized_module = module1.serialize()

# Let's forget about the module for this example.
del module1

# Deserialize the module.
module2 = Module.deserialize(serialized_module)

# Instantiate and use it.
result = module2.instantiate().exports.sum(1, 2)

print(result) # 3
```

A serialized module is a sequence of bytes. They can be saved in any
storage.

The `Module.validate` static method check whether the given bytes
represent valid WebAssembly bytes:

```python
from wasmer import Module

wasm_bytes = open('my_program.wasm', 'rb').read()

if not Module.validate(wasm_bytes):
    print('The program seems corrupted.')
```

## The `Value` class

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

# Integer on 128-bits.
value_v128 = Value.v128(7)
```

The `Value.([if](32|64)|v128)` static methods must be considered as
static constructors.

The `__repr__` method allows to get a string representation of a
`Value` instance:

```python
print(repr(value_i32)) # I32(7)
```

## The `Memory` class

A WebAssembly instance has its own memory, represented by the `Memory`
class. It is accessible by the `Instance.memory` getter.

### Growing the memory

The `Memory.grow` method allows to grow the memory by a number of
pages (of 65kb each).

```python
instance.memory.grow(1)
```

### Getting an access to the in-memory data

The `Memory` class offers 2 ways to get an access to its data:

  1. Direct raw buffer access, through the [Python Buffer
     Protocol][python-buffer-protocol],
  2. Views.

To get a direct raw buffer, you can use the `buffer` getter, combined
with the builtin [`memoryview`], [`bytes`] or [`bytearray`] Python
functions:

```python
# With `memoryview`
memory_view = memoryview(instance.memory.buffer)
memory_size = memory_view.nbytes
assert bytes(memory_view[0:3]).decode() == 'Wasmer'

# With `bytearray`
byte_array = bytearray(instance.memory.buffer)
memory_size = len(byte_array)
assert byte_array[0:6].decode() == 'Wasmer'
```

To create specific views over the memory data, you can use the
following methods:

  * `uint8_view(offset = 0)`,
  * `int8_view(offset = 0)`,
  * `uint16_view(offset = 0)`,
  * `int16_view(offset = 0)`,
  * `uint32_view(offset = 0)`,
  * `int32_view(offset = 0)`.

All these methods accept one optional argument: `offset`, to subset
the memory view at a particular offset. These methods return
respectively an `*Array` object, i.e. `uint8_view` returns a
`Uint8Array` object and so on.

```python
uint8_view = instance.memory.uint8_view(offset = 7)
bytes = uint8_view[0:3]
```

[python-buffer-protocol]: https://docs.python.org/3/c-api/buffer.html
[`memoryview`]: https://docs.python.org/3.3/library/functions.html#func-memoryview
[`bytes`]: https://docs.python.org/3.3/library/functions.html#bytes
[`bytearray`]: https://docs.python.org/3.3/library/functions.html#bytearray

### The `*Array` classes

These classes represent views over a memory of an instance where
elements are specific bytes.

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

### Let's see in action

First start with a `uint8_view`:

```python
from wasmer import Instance

# Get the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Instantiate the Wasm module.
instance = Instance(wasm_bytes)

# Call a function that returns a pointer to a string for instance.
pointer = instance.exports.return_string()

# Get the memory view, with the offset set to `pointer` (default is 0).
memory = instance.memory.uint8_view(offset = pointer)
memory_length = len(memory)

# Read the string pointed by the pointer.
nth = 0;
string = ''

while nth < memory_length:
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
print(bytes(memory[0:13]).decode()) # Hello, World!
```

With a direct raw buffer, we would get:

```python
# Call a function that returns a pointer to a string for instance.
pointer = instance.exports.return_string()

# Get a `bytearray` object.
byte_array = bytearray(instance.memory.buffer)

# Read the string pointed by the pointer.
print(byte_array[pointer:pointer+13].decode()) # Hello, World!
```

Notice that `*Array` and `Buffer` treat bytes in little-endian, as
required by the WebAssembly specification, [Chapter Structure, Section
Instructions, Sub-Section Memory
Instructions](https://webassembly.github.io/spec/core/syntax/instructions.html#memory-instructions):

> All values are read and written in [little
> endian](https://en.wikipedia.org/wiki/Endianness#Little-endian) byte
> order.

Each view shares the same memory buffer internally. Let's have some fun:

```python
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
assert int32[0] == 0b01000000_00010000_00000100_00000001
```

### Performance

Using the direct raw buffer API with
`bytearray(instance.memory.buffer)` is 15x faster than using
`instance.memory.uint8_view()` for _reading_. However, the direct raw
buffer API is _read-only_ for the moment, whilst the views are read
and write. Chose them wisely.

# Development

The Python extension is written in Rust, with [`pyo3`] and
[`maturin`].

To set up your environment, run only once:

```sh
$ just prelude
```

It will install `pyo3` and `maturin` for Python and for Rust. It will
also install [`virtualenv`].

Then, simply run:

```sh
$ .env/bin/activate
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

(Yes, you need [`just`]).

# Testing

Once the extension is compiled and installed (just run `just build`),
run the following command:

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
