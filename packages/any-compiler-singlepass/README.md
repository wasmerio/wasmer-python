# `wasmer-compiler-singlepass-any`

A special `wasmer-$(version)-py3-none-any` wheel is built as a
fallback. The `wasmer` libray will be installable, but it will raise
an `ImportError` exception saying that “Wasmer is not available on
this system”.

This wheel will be installed if none matches before (learn more by
reading the [PEP 425, Compatibility Tags for Built
Distributions](https://www.python.org/dev/peps/pep-0425/)).
