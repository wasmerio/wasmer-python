# Examples

This directory contains a collection of examples. This isn't an
exhaustive collection though, if one is missing, please ask, we will
be happy to fulfill your needs!

The examples are written in a difficulty/discovery order. Concepts
that are explained in an example is not necessarily re-explained in a
next example.

## Basics

1. [**Instantiating a module**][instance], explains the basics of
   using Wasmer and how to create an instance out of a WebAssembly
   module.
   
   _Keywords_: instance, module.
   
   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/instance.py
   ```

   </details>

## Engines

2. [**Universal engine**][engine-universal], explains what an engine
   is, what the Universal engine is, and how to set it up. The example
   completes itself with the compilation of the Wasm module, its
   instantiation, and finally, by calling an exported function.
   
   _Keywords_: universal, engine, in-memory, executable code.
   
   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_universal.py
   ```

   </details>

3. [**Dylib engine**][engine-dylib], explains what a Dylib engine
   is, and how to set it up. The example completes itself with the
   compilation of the Wasm module, its instantiation, and finally, by
   calling an exported function.
   
   _Keywords_: dylib, engine, shared library, dynamic library,
   executable code.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_dylib.py
   ```

   </details>

4. [**Headless engines**][engine-headless], explains what a headless
   engine is, what problem it does solve, and what are the benefits of
   it. The example completes itself with the instantiation of a
   pre-compiled Wasm module, and finally, by calling an exported
   function.
   
   _Keywords_: dylib, engine, constrained environment, ahead-of-time
   compilation, cross-compilation, executable code, serialization.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_headless.py
   ```

   </details>

5. [**Cross-compilation**][engine-cross-compilation], illustrates the
   power of the abstraction over the engines and the compilers, such
   as it is possible to cross-compile a Wasm module for a custom
   target.
   
   _Keywords_: engine, compiler, cross-compilation.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_cross_compilation.py
   ```

   </details>

## Compilers

6. [**Singlepass compiler**][compiler-singlepass], explains how to use
   the `wasmer_compiler_singlepass` compiler.
   
   _Keywords_: compiler, singlepass.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/compiler_singlepass.py
   ```

   </details>

7. [**Cranelift compiler**][compiler-cranelift], explains how to use
   the `wasmer-compiler-cranelift` compiler.
   
   _Keywords_: compiler, cranelift.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/compiler_cranelift.py
   ```

   </details>

8. [**LLVM compiler**][compiler-llvm], explains how to use the
   `wasmer-compiler-llvm` compiler.
   
   _Keywords_: compiler, llvm.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/compiler_llvm.py
   ```

   </details>

## Exports
   
9. [**Exported function**][exported-function], explains how to get and
   how to call an exported function.
   
   _Keywords_: export, function.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/exports_function.py
   ```

   </details>

10. [**Exported memory**][exported-memory], explains how to read from
    and write into an exported memory.

    _Keywords_: export, function.

    <details>
    <summary><em>Execute the example</em></summary>

    ```shell
    $ python examples/exports_memory.py
    ```

    </details>

11. [**Exported global**][exported-global], explains how to work with
    exported globals: get/set their value, have information about
    their type.

    _Keywords_: export, global.

    <details>
    <summary><em>Execute the example</em></summary>

    ```shell
    $ python examples/exports_global.py
    ```

    </details>

## Imports

12. [**Imported function**][imported-function], aka _host function_,
    explains how to use a Python function inside a WebAssembly module.

    _Keywords_: import, function.

    <details>
    <summary><em>Execute the example</em></summary>

    ```shell
    $ python examples/imports_function.py
    ```

    </details>

13. [**Early exit**][imported-function-early-exit], explains how to
    early exit from a host function, and how to catch the error later.

    _Keywords_: import, function, error.

    <details>
    <summary><em>Execute the example</em></summary>

    ```shell
    $ python examples/imports_function_early_exit.py
    ```

    </details>

## Integrations

14. [**WASI**][wasi], explains how to use the [WebAssembly System
    Interface][WASI] (WASI).
   
    _Keywords_: wasi, system, interface

    <details>
    <summary><em>Execute the example</em></summary>

    ```shell
    $ python examples/wasi.py
    ```

    </details>

[instance]: ./instance.py
[engine-universal]: ./engine_universal.py
[engine-dylib]: ./engine_dylib.py
[engine-headless]: ./engine_headless.py
[engine-cross-compilation]: ./engine_cross_compilation.py
[compiler-singlepass]: ./compiler_singlepass.py
[compiler-cranelift]: ./compiler_cranelift.py
[compiler-llvm]: ./compiler_llvm.py
[exported-function]: ./exports_function.py
[exported-memory]: ./exports_memory.py
[exported-global]: ./exports_global.py
[imported-function]: ./imports_function.py
[imported-function-early-exit]: ./imports_function_early_exit.py
[wasi]: ./wasi.py
