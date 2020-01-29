// Compiled with `wasmcc sum_c.c -o sum_c.wasm`.
// Check https://github.com/wasienv/wasienv.

#include <stdint.h>

int add_one(int32_t x) {
  return x + 1;
}
