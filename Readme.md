Well documented, safe-leaning, sound, low-level API bindings to `HWND`-adjacent APIs.

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/hwnd.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/hwnd)
[![crates.io](https://img.shields.io/crates/v/hwnd.svg)](https://crates.io/crates/hwnd)
[![docs.rs](https://docs.rs/hwnd/badge.svg)](https://docs.rs/hwnd)
[![License](https://img.shields.io/crates/l/hwnd.svg)](https://github.com/MaulingMonkey/hwnd)
[![Build Status](https://github.com/MaulingMonkey/hwnd/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/hwnd/actions?query=workflow%3Arust)



## Raison d'être

Intended use cases include:
*   Building block for higher level APIs.
*   Interop with existing win32-consuming C or C++ codebases (incrementally RIIRing?)
*   Direct use for cases where extra abstraction / higher level APIs are overkill.

Goals to service these use cases, in order of priority, are:

0.  APIs should be as *sound* as possible.
    *   Spam unit tests and/or doc-tests to try to break safe APIs.
    *   Thorough safety documentation of any fns or traits that must remain `unsafe`.
1.  Keep a 1:1 mapping to raw windows types and APIs as much as possible.
    *   Maximum interoperability with existing C or C++ codebases.
    *   Avoids infinite yak shaving.
    *   Additional error checking / debug spam is generally discouraged to avoid suprising RIIRers with new edge cases, except to avoid soundness issues.
2.  Make APIs as safe as possible.
    *   Some `unsafe` will generally still be required to implement said APIs.
    *   Eliminating `unsafe` for perfectly sound APIs will make auditing the remaining `unsafe` code easier.
3.  Provide excellent documentation.
    *   Rust specific examples for everything.
    *   Thoroughly document edge cases, error codes, etc.
    *   Aiming to be even better than MSDN.



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
