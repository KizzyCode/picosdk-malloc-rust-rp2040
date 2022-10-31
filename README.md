[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


# `picosdk-malloc`

This crate provides a heap-allocated container that uses the Pico-SDK's `malloc`/`free` for memory management.

This is useful if:
 - you link against the Pico SDK
 - you don't want to use `nightly`
 - you have large objects that cannot fit into the stack (the default stack size is rather small)
