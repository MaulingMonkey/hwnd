use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)\]
/// KillTimer
///
/// Cancels or removes a timer, identified by `(hwnd, id_event)`.
///
/// Note that this does *not* remove any [`WM::TIMER`] messages that were already enqueued.
///
/// ### Errors
/// Returns <code>[Err]\([KillTimerError]\(_\)\)</code> if <code>[TryInto]&lt;[HWnd]&gt;</code> failed or if `(hwnd, id_event)` didn't identify a timer.
///
/// [Microsoft's documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer#return-value) claims detailed errors can be acquired from [GetLastError] &mdash;
/// however, testing on Windows 10.0.19045.3803, this appears to be false ([GetLastError] returns stale errors from previous, likely unrelated API calls.)
///
/// [GetLastError]: https://learn.microsoft.com/en-us/windows/desktop/api/errhandlingapi/nf-errhandlingapi-getlasterror
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winapi::um::winuser::*;
/// # use winresult::*;
/// # use core::ptr::null_mut;
/// # use core::sync::atomic::*;
/// # let hinstance   = get_module_handle_entry_exe().unwrap();
/// # let name        = abistr::utf16ish!("kill_timer.docs");
/// # let class       = unsafe { register_class_w(&WndClassW { wnd_proc: Some(def_window_proc_w), hinstance, class_name: name.into(), .. Default::default() }) }.unwrap();
/// # let hwnd1       = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
/// # let hwnd2       = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
/// # let desktop     = get_desktop_window();
/// # let invalid     = HWnd::from(0x12345678 as HWND);
/// let _ = set_timerproc_exception_suppression(false); // don't suppress timer_func panic!s
///
/// unsafe {
///     // Each of these is a unique timer:
///     set_timer(None,  0,  100, Some(timer_func)).expect("(None, ???)");
///     set_timer(hwnd1, 42, 100, Some(timer_func)).expect("(hwnd1, 42)");
///     set_timer(hwnd2, 42, 100, Some(timer_func)).expect("(hwnd2, 42)");
/// }
///
/// // triggered 3x after 1/10th of a second (100 milliseconds):
/// extern "system" fn timer_func(hwnd: HWnd, _: WM32, id_event: usize, _: u32) {
///     # TIMER_FUNC_CALLS.fetch_add(1, Ordering::Relaxed);
///     # assert!(hwnd.is_null() || id_event == 42);
///     kill_timer(hwnd, id_event).unwrap(); // fire each timer only once
///     kill_timer(hwnd, id_event).unwrap_err(); // already killed
/// }
///
/// // ...message loop...
/// # unsafe { set_timer(None, 42, 300, Some(quit)) }.unwrap();
/// # extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
/// # static TIMER_FUNC_CALLS : AtomicU32 = AtomicU32::new(0);
/// # let mut msg = Msg::zeroed();
/// # while get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap() {
/// #     translate_message(&msg);
/// #     let _ = unsafe { dispatch_message_w(&msg) };
/// # }
/// #
/// # assert_eq!(3, TIMER_FUNC_CALLS.load(Ordering::Relaxed));
/// ```
///
/// ### See Also
/// *   [`set_timer`]
/// *   [`set_coalescable_timer`]
/// *   [`WM::TIMER`]
pub fn kill_timer(hwnd: impl TryInto<HWnd>, id_event: usize) -> Result<(), KillTimerError> {
    fn_context!(kill_timer => KillTimer);
    let hwnd = hwnd.try_into().map_err(|_| KillTimerError(()))?.into();
    match unsafe { KillTimer(hwnd, id_event) } { 0 => Err(KillTimerError(())), _ => Ok(()) }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)\]
/// [kill_timer] failed (`(hwnd, id_event)` wasn't a timer)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct KillTimerError(());

impl core::fmt::Display for KillTimerError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "KillTimer failed: `(hwnd, id_event)` wasn't a timer")
    }
}
