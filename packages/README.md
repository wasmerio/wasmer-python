# Wasmer Python Packages

Wasmer provides several Python packages. We recommend to read the
[`api/README.md`](./api/README.md) to get a complete explanations.

Succinctly, here is a short description of this directory:

* `any/` contains a fallback to make `wasmer` always installable on
  all platforms, but it could raise an `ImportError` if the current
  platform isn't supported,

* `api/` contains the `wasmer` package,

* `compiler-cranelift/` contains the Cranelift compiler, as the
  `wasmer_compiler_cranelift` package,

* `compiler-llvm/` contains the LLVM compiler, as the
  `wasmer_compiler_llvm` package,

* `compiler-singlepass/` contains the Singlepass compiler, as the
  `wasmer_compiler_singlepass` package,

* `engines/` contains code that is shared by `api/` and by the
  compilers.
