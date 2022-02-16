use bytemuck::*;

use winapi::shared::windef::HWND;

use std::convert::Infallible;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
/// HWND
///
/// A handle to a window.<br>
/// May or may not belong to the current thread.<br>
/// May or may not belong to the current *process*.<br>
/// May or may not have ever existed, or be valid.<br>
/// May be some kind of magic number or value, like HWND_PARENT.<br>
/// May be null.<br>
/// May be a top level application/main window.<br>
/// May be a tiny button.<br>
/// Recommended reading: [What Is a Window?](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-).
///
/// ### Safety: A Common Bug
/// *   You start handling a window message or rendering a scene with e.g. Direct3D.
/// *   You create a dialog.
///     *   Perhaps a C++ assertion dialog.
///     *   Perhaps a Win32 internal debug check.
///     *   Perhaps another debug check.
///     *   Perhaps a system file dialog.
///     *   Perhaps indirectly triggered by middleware or callback.
/// *   The dialog might not specify the window being processed/rendered as a parent.
/// *   The dialog starts it's own message loop, processing window messages, including for the parent window you were in the middle of processing/rendering.
/// *   The user can then Alt+F4, click X, or otherwise close the window being processed/rendered.
/// *   This destroys the window.
/// *   The user then closes the dialog.
/// *   Your code continues on handling a window message, or rendering a scene, with a dangling [HWnd].
/// *   ❌ **Undefined behavior ensues** ❌
///
/// ### Safety: More Formally
/// [HWnd]s are fundamentally data-racey, and fragile.
///
/// *   [HWnd]s are [Send]+[Sync], as the underlying window, while ! [Send], is [Sync].
///     *   Some window state is Atomic or internally locked (might be updated by any other thread at any time.)
///     *   Some window state is thread-local / guarded (e.g. you can't destroy a window from another thread, but you *can* send it a WM_CLOSE request/message.)
///     *   Some window state is conceptually UnsafeCell (e.g. wndprocs can be changed from another thread, but there is no way to safely do so!)
///     *   Ref: [Thread affinity of user interface objects, part 1: Window handles](https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843)
/// *   [HWnd]s are *weak* references that *cannot* be properly locked.
///     *   You can test if a window is local and valid with e.g. [get_window_thread_id]\(hwnd\)? == [get_current_thread_id]\(\).
///         *   This can be used to ensure e.g. Device creation is sound.
///         *   This alone *cannot* ensure an [HWnd] isn't destroyed out from under the Device!
///     *   Any jerk can [destroy_window] the window you're currently processing.
///     *   [destroy_window] cannot be blocked/canceled.
///     *   [destroy_window] *can* be hooked - either by system hooks, or by overriding the wndproc to listen for WM_DESTROY.
///     *   [destroy_window] hooks can hang (`loop {}`), or [abort](std::process::abort) on debug-locks of the [HWnd] to ensure soundness.
///     *   Sufficiently evil (ab)use of unsafe Win32 APIs can cause your hooks to be skipped, but I think such abuse can be considered
///         soundness bugs in the abusing code, and that it is "sound" for your own hooks to rely on such bugs not existing in other code.
///
/// ### Safety: Recommended Mitigations
/// *   Avoid recursive rendering for e.g. OpenGL / Direct3D, by either:
///     *   Rendering from your top message loop, not from `WM_PAINT`
///     *   Explicitly guarding against rendering recursion.
/// *   Instead of immediately destroying Direct3D windows, hide them:
///     *   Don't call [destroy_window]
///     *   Don't call DefWindowProcW for WM_CLOSE either, as that will [destroy_window]!
///     *   [destroy_window] them later in your main loop when not rendering nor inside your wndproc.
/// *   Use a wndproc or hook to ensure you process WM_DESTROY.  To process:
///     *   Release your graphics context from WM_DESTROY if possible, by hooks if needed.
///     *   If that fails, hang (`loop {}`), or [abort](std::process::abort) to ensure soundness / avoid undefined behavior.
///         *   A user-friendly message (perhaps another dialog!) before hanging/aborting is recommended, instead of just silently closing the window.
///         *   Might happen if e.g. a graphics debugger or middleware is holding onto an extra device reference.
///         *   Panics are insufficient for soundness: they can be caught, and it's AFAIK unsound to panic over FFI boundaries.
///
/// ### See Also
/// *   [What Is a Window?](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)
/// *   [Thread affinity of user interface objects, part 1: Window handles](https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
#[repr(transparent)]
pub struct HWnd(pub(crate) usize);

impl HWnd {
    pub const fn is_null(self) -> bool { self.0 == 0 }
}

impl Debug for HWnd { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HWnd(0x{:X})", self.0) } }

impl From<()>                   for HWnd { fn from(_: ()                 ) -> Self { Self(0) } }
impl From<Option<Infallible>>   for HWnd { fn from(_: Option<Infallible> ) -> Self { Self(0) } }
impl From<HWnd>                 for HWND { fn from(h: HWnd               ) -> Self { h.0 as _ } }
impl From<HWND>                 for HWnd { fn from(h: HWND               ) -> Self { Self(h as _) } }
