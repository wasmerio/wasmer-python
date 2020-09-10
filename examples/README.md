# Examples

This directory contains a collection of examples. This isn't an
exhaustive collection though, if one is missing, please ask, we will
be happy to fulfill your needs!

The examples are written in a difficulty/discovery order. Concepts
that are explained in an example is not necessarily re-explained in a
next example.

## Engines

1. [**JIT engine**][engine-jit], explains what an engine is, what the
   JIT engine is, and how to set it up. The example completes itself
   with the compilation of the Wasm module, its instantiation, and
   finally, by calling an exported function.
   
   _Keywords_: JIT, engine, in-memory, executable code.
   
   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_jit.py
   ```

   </details>

2. [**Native engine**][engine-native], explains what a native engine
   is, and how to set it up. The example completes itself with the
   compilation of the Wasm module, its instantiation, and finally, by
   calling an exported function.
   
   _Keywords_: native, engine, shared library, dynamic library,
   executable code.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_native.py
   ```

   </details>

3. [**Headless engines**][engine-headless], explains what a headless
   engine is, what problem it does solve, and what are the benefits of
   it. The example completes itself with the instantiation of a
   pre-compiled Wasm module, and finally, by calling an exported
   function.
   
   _Keywords_: native, engine, constrained environment, ahead-of-time
   compilation, cross-compilation, executable code, serialization.

   <details>
   <summary><em>Execute the example</em></summary>

   ```shell
   $ python examples/engine_headless.py
   ```

   </details>

[engine-jit]: ./engine_jit.py
[engine-native]: ./engine_native.py
[engine-headless]: ./engine_headless.py
