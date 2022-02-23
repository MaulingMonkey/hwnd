Rust code linking third party C or C++ cannot be sound, unless it is allowed to make assumptions about said C or C++.
Those assumptions can be incredibly implicit - this document attempts to make those assumptions explicit.



### DLL Implementations

Windows Versions:
*   rust-lang supports Windows 7+ ([ref](https://doc.rust-lang.org/nightly/rustc/platform-support.html)), so this crate should too.
*   I make no commitment to this crate being sound on earlier versions of windows!
*   **Windows before 10 is poorly tested**, but patches are welcome.

Non-Windows Versions:
*   WINE:       untested, any "extra" soundness requirements vs Win7+ should probably be fixed in WINE itself.
*   ReactOS:    untested, any "extra" soundness requirements vs Win7+ should probably be fixed in ReactOS itself.

Third Party DLLs:
*   If a third party hooks / detours system DLL functions, any extra soundness requirements are arguably bugs in said third party hooks / detours, not in this crate.
*   I *might* be convinced to workaround third party bugs in some cases, but don't hold your breath.



### Hook Implementations

Correctly managing the lifetime of HWND-associated data involves relying on hooks being called correctly.
[SetWindowsHookExW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
based hooks require all their implementors to properly call
[CallNextHookEx](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex)
to ensure all hooks are called.
Technically speaking, there's no guarantee this will occur.
However, arbitrary C++ code (e.g. MFC) is reasonably likely to rely on the proper functioning of hooks to avoid undefined behavior.
As such, **I declare by fiat that misbehaving hooks are soundness issues unto themselves,**
and that Rust code "should" be able to rely on the proper execution of said hooks for soundness.

Of course, extra debug checks making a best effort to catch and diagnose failures of said hooks wouldn't hurt.
