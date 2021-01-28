# <img height="48" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/assets/logo.png" alt="Wasmer logo" valign="middle"> Wasmer Python, the Singlepass compiler [![PyPI version](https://img.shields.io/pypi/v/wasmer_compiler_singlepass)](https://pypi.org/project/wasmer_compiler_singlepass/) [![Wasmer Python Documentation](https://img.shields.io/badge/docs-read-green)](https://wasmerio.github.io/wasmer-python/api/wasmer_compiler_singlepass/) [![Wasmer PyPI downloads](https://pepy.tech/badge/wasmer_compiler_singlepass)](https://pypi.org/project/wasmer_compiler_singlepass/) [![Wasmer Slack Channel](https://img.shields.io/static/v1?label=chat&message=on%20Slack&color=green)](https://slack.wasmer.io)

`wasmer` is a complete and mature WebAssembly runtime for
Python. `wasmer_compiler_singlepass` provides the Singlepass compiler
to `wasmer`, so that `wasmer` is able to compile WebAssembly module.

Other compilers exist:

* `wasmer_compiler_cranelift`,
* `wasmer_compiler_llvm`.

To get a more complete view, please see the documentation of [the
`wasmer` package itself](https://github.com/wasmerio/wasmer-python).

# Install

This package must be used with the `wasmer` package, thus:

```sh
$ pip install wasmer
$ pip install wasmer_compiler_singlepass
```

# Usage

Any engines in `wasmer.engine` can take the
`wasmer_compiler_singlepass.Compiler` class as argument:

```py
from wasmer import engine, Store, Module, Instance
from wasmer_compiler_singlepass import Compiler

# Let's use the Singlepass compiler with the JIT engine…
store = Store(engine.JIT(Compiler))

# … or with the native engine!
store = Store(engine.Native(Compiler))

# And now, compile the module.
module = Module(store, open('my_program.wasm', 'rb').read())

# Now it's compiled, let's instantiate it.
instance = Instance(module)

# And get fun, for example by calling the `sum` exported function!
print(instance.exports.sum(1, 2))
```

# Documentation

Browse the documentation at
https://wasmerio.github.io/wasmer-python/api/wasmer_compiler_singlepass/.

Alternatively, run `just build compiler-singlepass` followed by `just
doc` to generate the documentation inside
`docs/api/wasmer_compiler_singlepass.html`.
