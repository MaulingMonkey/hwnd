Alternative crates covering a similar API surface to compare/contrast against.

# Alternatives
*   [winit](https://lib.rs/crates/winit):                       *cross platform* windowing.  Limited (but present) native interop, compared to this crate.
*   [winapi](https://lib.rs/crates/winapi):                     *raw FFI*.  Sound, but no attempt at docs or safety, compared to this crate.  Common to misuse with dangling string pointers.
*   [windows](https://lib.rs/crates/windows):                   *raw FFI*.  Sound, but no attempt at docs or safety (for hwnd-adjacent APIs), compared to this crate.
*   [winsafe](https://lib.rs/crates/winsafe):                   Similar goals!  Less focus on exhaustive testing, documentation, and sound definitions - but more attempts at higher level and safe APIs, leading to sound usage.
*   [lib.rs/gui](https://lib.rs/gui):                           Other GUI crates
*   [lib.rs/os/windows-apis](https://lib.rs/os/windows-apis):   Other Windows crates

<!-- [#24](https://github.com/rodrigocfd/winsafe/issues/24), [#25](https://github.com/rodrigocfd/winsafe/issues/25), [#26](https://github.com/rodrigocfd/winsafe/issues/26) -->

Ultimately, none of these scratch my *exact* itch.  My goals, in order of priority, are:

0.  APIs should be as *sound* as possible.
    *   Spam unit tests and/or doc-tests to try to break APIs.
    *   Thorough safety documentation of any fns or traits that must be `unsafe`.
1.  Keep a 1:1 mapping to raw windows types and APIs as much as possible.
    *   Maximum interoperability with existing C or C++ codebases.
    *   Avoids infinite yak shaving.
    *   Additional error checking / debug spam is generally discouraged to avoid suprising RIIRers with new edge cases, except to avoid soundness issues.
2.  Make APIs as safe as possible.
3.  Provide excellent documentation
    *   Rust specific examples for everything.
    *   Thoroughly document edge cases, error codes, etc.
    *   Aiming to be even better than MSDN.

These goals are intended to make this crate useful for a few scenarios:
*   Building higher level, safe, sound APIs / abstractions in Rust.
    *   Some `unsafe` will generally still be required to implement said APIs.
    *   Eliminating `unsafe` for perfectly sound APIs will make auditing the remaining `unsafe` code easier.
*   Direct use for cases where extra abstraction / higher level APIs are overkill.
*   Easier incremental rewrites of an existing C or C++ codebase.
    *   Converting to/from abstraction types is often quite awkward at best.
