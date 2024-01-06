use crate::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)\]
/// TIMERPROC
/// &mdash;
/// A non-null callback for <code>[set](set_timer)\[[_coalescable](set_coalescable_timer)\][_timer](set_timer)</code>.
///
/// Note that exceptions may be discarded/ignored when this is called by a Windows timer, unless you first call <code>[set_timerproc_exception_suppression]\(false\)</code>, as recommended by Microsoft.
///
/// ### Parameters
/// *   `hwnd`                      - The [`HWnd`] associated with this timer.
/// *   `msg`                       - The message type associated with this timer (e.g. [`WM::TIMER`].)
/// *   `id_event`                  - The same `id_event` that was passed to <code>[set](set_timer)\[[_coalescable](set_coalescable_timer)\][_timer](set_timer)</code>.
/// *   `tick_count_ms`             ≈ [`GetTickCount()`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount) (the number of milliseconds since system start, wrapping every 49.7 days.)
///
/// ### See Also
/// *   [`set_timer`]               - Accepts an <code>[Option]&lt;[TimerProcNonNull]&gt;</code>.
/// *   [`set_coalescable_timer`]   - Accepts an <code>[Option]&lt;[TimerProcNonNull]&gt;</code>.
/// *   [`kill_timer`]              - Can be passed `hwnd`, `id_event` to stop repeating this callback.
/// *   [`WM::TIMER`]               - An alternative to this callback.
/// *   [`TimerProc`]               - The [`Option`]able/nullable alternative to this callback.
pub type TimerProcNonNull = extern "system" fn (hwnd: HWnd, msg: WM32, id_event: usize, tick_count_ms: u32);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)\]
/// TIMERPROC
/// &mdash;
/// An [`Option`]al callback for <code>[set](set_timer)\[[_coalescable](set_coalescable_timer)\][_timer](set_timer)</code>.
///
/// Note that exceptions may be discarded/ignored when this is called by a Windows timer, unless you first call <code>[set_timerproc_exception_suppression]\(false\)</code>, as recommended by Microsoft.
///
/// ### Parameters
/// *   `hwnd`                      - The [`HWnd`] associated with this timer.
/// *   `msg`                       - The message type associated with this timer (e.g. [`WM::TIMER`].)
/// *   `id_event`                  - The same `id_event` that was passed to <code>[set](set_timer)\[[_coalescable](set_coalescable_timer)\][_timer](set_timer)</code>.
/// *   `tick_count_ms`             ≈ [`GetTickCount()`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount) (the number of milliseconds since system start, wrapping every 49.7 days.)
///
/// ### See Also
/// *   [`set_timer`]               - Accepts an [`TimerProc`].
/// *   [`set_coalescable_timer`]   - Accepts an [`TimerProc`].
/// *   [`kill_timer`]              - Can be passed `hwnd`, `id_event` to stop repeating this callback.
/// *   [`WM::TIMER`]               - An alternative to this callback.
/// *   [`TimerProcNonNull`]        - The non-null alternative to this callback.
pub type TimerProc = Option<TimerProcNonNull>;
