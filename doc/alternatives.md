Alternative crates covering a similar API surface to compare/contrast against.

# Alternatives
*   [winit](https://lib.rs/crates/winit):                       *cross platform* windowing.  Limited (but present) native interop, compared to this crate.
*   [winapi](https://lib.rs/crates/winapi):                     *raw FFI*.  Sound, but no attempt at docs or safety, compared to this crate.  Common to misuse with dangling string pointers.
*   [windows](https://lib.rs/crates/windows):                   *raw FFI*.  Sound, but no attempt at docs or safety (for hwnd-adjacent APIs), compared to this crate.
*   [winsafe](https://lib.rs/crates/winsafe):                   Similar goals!  Less focus on exhaustive testing, documentation, and sound definitions - but more attempts at higher level and safe APIs, leading to sound usage.
*   [lib.rs/gui](https://lib.rs/gui):                           Other GUI crates
*   [lib.rs/os/windows-apis](https://lib.rs/os/windows-apis):   Other Windows crates

<!-- [#24](https://github.com/rodrigocfd/winsafe/issues/24), [#25](https://github.com/rodrigocfd/winsafe/issues/25), [#26](https://github.com/rodrigocfd/winsafe/issues/26) -->
