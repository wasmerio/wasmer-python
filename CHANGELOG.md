# Changelog

All notable changes to this project will be documented in this file.

## Table of Contents

* [1.0.0-beta1](#100-beta1---2020-12-03)
* [0.4.1](#041---2020-02-02)
* [0.3.0](#030---2019-07-16)
* [0.2.0](#020---2019-04-16)

## [1.0.0-beta1] - 2020-12-03

### Changed

* The whole API changed to better match Wasmer and Wasm C API

  ```python
  from wasmer import engine, wat2wasm, Module, Store, Instance
  from wasmer_compiler_cranelift import Compiler
  
  # Create an Engine
  jit = engine.JIT(Compiler)
  
  # Create a store.
  store = Store(jit)
  
  # Let's compile the Wasm module.
  module = Module(store, wasm_bytes)
  
  # Create an empty import object.
  import_object = {}
  
  # Let's instantiate the Wasm module.
  instance = Instance(module, import_object)
  ```

  Please refer to the [examples](examples) and 
  [documentation](https://docs.wasmer.io/integrations/python) to learn more about the changes.

## [0.4.1] - 2020-02-02

### Added

* New `Buffer` class to read memory fast
  ([#125](https://github.com/wasmerio/wasmer-python/pull/125) by
  [@Hywan])
  
  To get the memory buffer, use the `Memory.buffer` getter. A `Buffer`
  implements the [Python Buffer
  Protocol](https://docs.python.org/3/c-api/buffer.html). The goal is
  to get faster reading operations than the existing memory views API.
  
  `bytearray(instance.memory.buffer)` is 15x faster than `instance.memory.int8_view()`.
  `memoryview(instance.memory.buffer)` is 14x faster than `instance.memory.int8_view()`.
  
  ```python
  # Get the memory buffer.
  buffer = Instance(wasm_bytes).memory.buffer

  # Use the buffer with `memoryview`
  memory_view = memoryview(buffer)

  # Use the buffer with `byte_array`
  byte_array = bytearray(buffer)

  # Enjoy the byte array API!
  assert byte_array[3:9].decode() == 'Wasmer'
  ```

* Support exported globals through the `Instance.globals` API
  ([#120](https://github.com/wasmerio/wasmer-python/pull/120) by
  [@Hywan])
  
  ```python
  instance = Instance(wasm_bytes)
  x = instance.globals.x

  assert x.value == 7
  assert x.mutable == True

  x.value = 153

  assert x.value == 153
  ```

* Implement a WebAssembly custom section query API
  ([#118](https://github.com/wasmerio/wasmer-python/pull/118) by
  [@Hywan])
  
  `Module.custom_section_names` is used to list all the custom section
  names.

  `Module.custom_section` is used to read the value of a specific
  custom section. If the custom section does not exist, `None` is
  returned.

  ```python
  assert Module(wasm_bytes).custom_section('hello') == b'World!'
  ```

* Add the `Module.imports` getter to list all imports, and introduce
  the `ImportKind` enum
  ([#117](https://github.com/wasmerio/wasmer-python/pull/117) by
  [@Hywan])

  ```python
  assert Module(wasm_bytes).imports == [
      {
          'kind': ImportKind.FUNCTION,
          'namespace': 'ns',
          'name': 'f1',
      },
      {
          'kind': ImportKind.FUNCTION,
          'namespace': 'ns',
          'name': 'f2',
      },
      {
          'kind': ImportKind.MEMORY,
          'namespace': 'ns',
          'name': 'm1',
          # Additional pairs specific to `MEMORY`
          'minimum_pages': 3,
          'maximum_pages': 4,
      },
      {
          'kind': ImportKind.GLOBAL,
          'namespace': 'ns',
          'name': 'g1',
          # Additional pairs specific to `GLOBAL`
          'mutable': False,
          'type': 'f32'
      },
      {
          'kind': ImportKind.TABLE,
          'namespace': 'ns',
          'name': 't1',
          # Additional pairs specific to `TABLE`
          'minimum_elements': 1,
          'maximum_elements': 2,
          'element_type': 'anyfunc',
      }
  ]
  ```

* Add the `Module.exports` getter to list all exports, and introduce
  the `ExportKind` enum
  ([#115](https://github.com/wasmerio/wasmer-python/pull/115) and
  [#116](https://github.com/wasmerio/wasmer-python/pull/116) by
  [@Hywan])

  ```python
  assert Module(wasm_bytes).exports == [
      {
          'name': 'memory',
          'kind': ExportKind.MEMORY,
      },
      {
          'name': '__heap_base',
          'kind': ExportKind.GLOBAL,
      },
      {
          'name': '__data_end',
          'kind': ExportKind.GLOBAL,
      },
      {
          'name': 'sum',
          'kind': ExportKind.FUNCTION,
      },
  ]
  ```

* Support modules without an exported memory
  ([#114](https://github.com/wasmerio/wasmer-python/pull/114) by
  [@Hywan])

  ```python
  instance = Instance(wasm_bytes)

  # Now the `memory` getter can return `None`
  assert instance.memory == None
  ```

* Add Rust trait to allow inspection of exported functions
  ([#71](https://github.com/wasmerio/wasmer-python/pull/71) by
  [@Mec-iS])

  ```python
  instance = Instance(wasm_bytes)
  assert isinstance(instance.exports.sum.getfullargspec, str)
  assert isinstance(instance.exports.sum.getargs, str)
  ```

* Memory views support slice assignment
  ([#63](https://github.com/wasmerio/wasmer-python/pull/63) by
  [@Hywan]).

  ```python
  memory = instance.memory.uint8_view()
  memory[0:4] = b"abcd"
  ```

  The slice is bound to the memory view length. The slice accepts
  start, stop, and step parameters, so it is possible to write
  `view[0:5:2]` for instance. There is a huge difference with list
  slice assignment in Python: Elements in memory cannot be moved, so
  the assignment only overwrite elements.

  ```python
  // With regular Python list
  a = [1, 2, 3, 4, 5]
  a[1:3] = [10, 11, 12]

  assert a == [1, 10, 11, 12, 4, 5]

  // With WebAssembly memory views
  view[0:5] = [1, 2, 3, 4, 5]
  view[1:3] = [10, 11, 12]

  assert view[0:5] == [1, 10, 11, 4, 5]
  ```

  It is 10 times faster than a regular loop to write data in memory.

  Read the pull request to learn more.

* Make wasmer silently available anywhere with `wasmer-any`
  ([#62](https://github.com/wasmerio/wasmer-python/pull/62) by
  [@syrusakbary])

### Changed

* Improve documentation of `Memory`
  ([#127](https://github.com/wasmerio/wasmer-python/pull/127) by
  [@Hywan])
* Add a C to WebAssembly example in the documentation
  ([#122](https://github.com/wasmerio/wasmer-python/pull/122/) by
  [@Hywan])
* Explain new features
  ([#119](https://github.com/wasmerio/wasmer-python/pull/119) by
  [@Hywan])
* Migrate to Github Actions for the CI
  ([#97](https://github.com/wasmerio/wasmer-python/pull/97),
  [#98](https://github.com/wasmerio/wasmer-python/pull/98)
  [#99](https://github.com/wasmerio/wasmer-python/pull/99) and
  [#101](https://github.com/wasmerio/wasmer-python/pull/101) by
  [@Hywan])
* Update Wasmer from 0.6.0 to 0.14
  ([#70](https://github.com/wasmerio/wasmer-python/pull/70),
  [#80](https://github.com/wasmerio/wasmer-python/pull/80),
  [#88](https://github.com/wasmerio/wasmer-python/pull/88),
  [#95](https://github.com/wasmerio/wasmer-python/pull/95),
  [#113](https://github.com/wasmerio/wasmer-python/pull/113) and
  [#132](https://github.com/wasmerio/wasmer-python/pull/132) by
  [@Hywan])
* Update Pyo3 from 0.8.2 to 0.8.4
  ([#93](https://github.com/wasmerio/wasmer-python/pull/93),
  [#96](https://github.com/wasmerio/wasmer-python/pull/96))

### Security

* Bump `spin` from 0.5.0 to 0.5.2
  ([#72](https://github.com/wasmerio/wasmer-python/pull/72)]

## [0.3.0] - 2019-07-16

### Added

* Add the `Memory.grow` method
  ([#56](https://github.com/wasmerio/wasmer-python/pull/56) by
  [@Hywan])
* Bound slice to the size of the memory view —allow to write
  `memory_view[0:]` with no error—
  ([#54](https://github.com/wasmerio/wasmer-python/pull/54) by
  [@Hywan])
* Add `wasmer.__core_version__` to get the runtime [Wasmer] version
  ([#51](https://github.com/wasmerio/wasmer-python/pull/51) by
  [@Hywan])
* Support module serialization with `Module.serialize` and
  `Module.deserialize`
  ([#48](https://github.com/wasmerio/wasmer-python/pull/48) by
  [@Hywan])

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
  ```

* Introduce the `Module` class, with `Module.validate` and
  `Module.instantiate`
  ([#47](https://github.com/wasmerio/wasmer-python/pull/47) by
  [@Hywan])

  ```python
  from wasmer import Module

  # Get the Wasm bytes.
  wasm_bytes = open('my_program.wasm', 'rb').read()

  # Compile the Wasm bytes into a Wasm module.
  module = Module(wasm_bytes)

  # Instantiate the Wasm module.
  instance = module.instantiate()

  # Call a function on it.
  result = instance.exports.sum(1, 2)

  print(result) # 3
  ```

* Add `wasmer.__version__` to get the extension version
  ([#27](https://github.com/wasmerio/wasmer-python/pull/27) by
  [@Mec-iS])

### Changed

* Handle exported functions that return nothing, aka void functions
  ([#38](https://github.com/wasmerio/wasmer-python/pull/38) by
  [@Hywan])
* More Python idiomatic examples
  ([#55](https://github.com/wasmerio/wasmer-python/pull/55) by
  [@Hywan])
* Add the `greet` example
  ([#43](https://github.com/wasmerio/wasmer-python/pull/43) by
  [@Hywan])
* Improve code documentation
  ([#36](https://github.com/wasmerio/wasmer-python/pull/36) by
  [@Mec-iS])
* Fix typos
  ([#25](https://github.com/wasmerio/wasmer-python/pull/25) by
  [@Hywan])
* Fix comments in examples
  ([#19](https://github.com/wasmerio/wasmer-python/pull/19) by
  [@Hywan])
* Setup Bors
  ([#35](https://github.com/wasmerio/wasmer-python/pull/35) by
  [@Hywan])
* Add Github templates
  ([#31](https://github.com/wasmerio/wasmer-python/pull/31) by
  [@Hywan])
* Rename `just rust` to `just build`
  ([#30](https://github.com/wasmerio/wasmer-python/pull/30) by
  [@Hywan])
* Update Wasmer to 0.5.5
  ([#59](https://github.com/wasmerio/wasmer-python/pull/59) by
  [@Hywan])
* Update Wasmer to 0.4.2
  ([#42](https://github.com/wasmerio/wasmer-python/pull/42) by
  [@Hywan])
* Update Wasmer to 0.4.0
  ([#26](https://github.com/wasmerio/wasmer-python/pull/26) by
  [@Hywan])

### Fixes

* Update smallvec
  ([#52](https://github.com/wasmerio/wasmer-python/pull/52) by
  [@Hywan])
* Update pyo3
  ([#46](https://github.com/wasmerio/wasmer-python/pull/46) by
  [@Hywan])

## [0.2.0] - 2019-04-16

[1.0.0-beta1]: https://github.com/wasmerio/wasmer-python/compare/0.4.1...1.0.0-beta1
[0.4.1]: https://github.com/wasmerio/wasmer-python/compare/0.3.0...0.4.1
[0.3.0]: https://github.com/wasmerio/wasmer-python/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/wasmerio/wasmer-python/compare/0.1.0...0.2.0
[@Hywan]: https://github.com/Hywan
[@Mec-iS]: https://github.com/Mec-iS
[@syrusakbary]: https://github.com/syrusakbary
