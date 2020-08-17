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

Table of Contents:

* [The `Instance` class](#the-instance-class)
  * [Exported functions](#exported-functions)
  * [Exported memory](#exported-memory)
  * [Imported functions](#imported-functions)
* [The `Module` class](#the-module-class)
  * [Exports, imports, and custom sections](#exports-imports-and-custom-sections)
  * [Serialization and deserialization](#serialization-and-deserialization)
* [The `ImportObject` class](#the-importobject-class)
* [The `Value` class](#the-value-class)
* [The `Memory` class](#the-memory-class)
  * [Growing the memory](#growing-the-memory)
  * [Getting an access to the in-memory data](#getting-an-access-to-the-in-memory-data)
  * [The `*Array` classes](#the-array-classes)
  * [Let's see in action](#lets-see-in-action)
  * [Performance](#performance)
* [WASI](#wasi)
* [`wat`](#wat)

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

### Imported functions

A WebAssembly module can _import_ functions, also called host
functions. It means that the implementation lands in the host, not in
the module. This feature is, for the moment, only supported on Unix
platforms, with a x86-64 architecture (it means that Windows is not
supported). Use `Features.host_functions()` to detect whether this
feature is available on your system.

Example of a Rust program that defines a `sum_plus_one` exported
function, and a `sum` imported function:

```rust
extern "C" {
    // This function is defined somehwere else.
    fn sum(x: i32, y: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn sum_plus_one(x: i32, y: i32) -> i32 {
    unsafe { sum(x, y) + 1 }
}
```

An imported function is defined by a namespace and a name. It is
defined by a Python dictionary, as follows:

```python
from wasmer import Instance

# Read the Wasm module as bytes.
wasm_bytes = open('my_program.wasm', 'rb').read()

# Compile those bytes into a real Wasm module.
module = Module(wasm_bytes)

# Declare the `sum` function, which will be the implementation
# for the `env.sum` imported function.
def sum(x: int, y: int) -> int:
    return x + y

# Create the import object for this module.
import_object = module.generate_import_object()
import_object.extend({
    "env": {
        "sum": sum
    }
})

# Instantiate the Wasm module, with the import object.
instance = Instance(wasm_bytes, import_object)

result = instance.exports.sum_plus_one(1, 2)

print(result) # 4
```

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

The `instantiate` method has an optional `import_object` argument. It
can be of type `ImportObject` or a Python dictionary. To generate an
import object, use the `generate_import_object` method.

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

## The `ImportObject` class

The `ImportObject` class aims at holding information relative to
[WASI](#wasi) or to [host functions](#imported-functions). To get an
instance of it, the user must use the
`Module.generate_import_object()` method, or the
`Wasi.generate_import_object_for_module(module)` method.

After this operation, the user has a pre-configured `ImportObject`. It
is then possible to extend it to declare host functions, like so:

```py
import_object = module.generate_import_object()
import_object.extend({
    'env': {
        'sum': sum
    }
})
```

It is also possible to list all the _declared_ import descriptors in
the `ImportObject` with:

```py
print(import_object.import_descriptors())
```

Finally, pass the `ImportObject` object into
`Module.instantiate(import_object)` or `Instance(wasm_bytes,
import_object)` to import all your entities.

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

The `Value.(i32|i64|f32|f64|v128)` static methods must be considered as
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
assert bytes(memory_view[0:6]).decode() == 'Wasmer'

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

## [WASI](https://github.com/WebAssembly/WASI)

This project supports WASI `Snapshot0` and `Snapshot1`, i.e. all the
WASI versions at the time of writing. To use WASI, please use the
`Wasi` class to build a _WASI state_. Then, based on this object,
create an `ImportObject` object, which can optionally be extended to
add [host functions](#imported-functions), to finally be passed to
`Module.instantiate` or `Instance`, and that's it.

Let's try with the following Rust program that prints various
information like program arguments, environment variables, and prints
the root of the file system:

```rust
use std::{env, fs};

fn main() {
    // Arguments
    {
        let mut arguments = env::args().collect::<Vec<String>>();

        println!("Found program name: `{}`", arguments[0]);

        arguments = arguments[1..].to_vec();
        println!(
            "Found {} arguments: {}",
            arguments.len(),
            arguments.join(", ")
        );
    }

    // Environment variables
    {
        let mut environment_variables = env::vars()
            .map(|(arg, val)| format!("{}={}", arg, val))
            .collect::<Vec<String>>();

        println!(
            "Found {} environment variables: {}",
            environment_variables.len(),
            environment_variables.join(", ")
        );
    }

    // Directories.
    {
        let root = fs::read_dir("/")
            .unwrap()
            .map(|e| e.map(|inner| format!("{:?}", inner)))
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        println!(
            "Found {} preopened directories: {}",
            root.len(),
            root.join(", ")
        );
    }
}
```

Let's compile this program to WebAssembly with WASI:

```sh
$ rustc --target wasm32-wasi -O wasi.rs -o wasi.raw.wasm
$ wasm-strip wasi.raw.wasm
$ wasm-opt -O4 -Oz wasi.raw.wasm -o wasi.wasm
$ rm wasi.raw.wasm
```

And now let's use it in Python!

```py
from wasmer import Module, Wasi, WasiVersion

# Compile the Wasm module.
wasm_bytes = open('wasi.wasm', 'rb').read()
module = Module(wasm_bytes)

# Assert that it is a WASI module, and look at the WASI version.
assert module.is_wasi_module == True
assert module.wasi_version() == WasiVersion.Snapshot1

# Create the WASI object.
wasi = Wasi(
    'wasi_test_program',
    arguments=['--test'],
    environments={"COLOR": "true", "APP_SHOULD_LOG": "false"},
    map_directories={"the_host_current_dir": "."},
)

# Get an `ImportObject` object from the `Wasi` object.
import_object = wasi.generate_import_object_for_module(module)

# Instantiate the module with the import object.
instance = module.instantiate(import_object)

# Have fun!
instance.exports._start()
```

When running this Python program, we will see the following output:

```txt
Found program name: `wasi_test_program`
Found 1 arguments: --test
Found 2 environment variables: COLOR=true, APP_SHOULD_LOG=false
Found 1 preopened directories: DirEntry("/the_host_current_dir")
```

It showcases that WebAssembly with WASI can access to `stdout`, can
have an environment (with program name, arguments, and environment
variables), and can have a restricted access to the file system.

## `wat`

The `wat2wasm` and `wasm2wat` functions respectively translates
WebAssembly text source to WebAssembly binary format, and disassembles
WebAssembly binary to WebAssembly text format. An example with
`wat2wasm`:

```py
from wasmer import wat2wasm, Instance

wat = """ (module
            (type (func (param i32 i32) (result i32)))
            (func (type 0)
                local.get 0
                local.get 1
                i32.add)
            (export "sum" (func 0))) """
wasm_bytes = wat2wasm(wat)
instance = Instance(wasm_bytes)

assert instance.exports.sum(1, 2) == 3
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
