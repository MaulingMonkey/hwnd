use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcoalescabletimer)\]
/// SetCoalescableTimer
///
/// Creates or replaces a timer that calls [`TimerProc`] every `elapse_ms` milliseconds.
///
/// Each timer is uniquely identified by `(hwnd, id_event)`.
///
/// ### Safety
/// This is *almost* safe.  Perhaps even *arguably* safe.
///
/// Arbitrary C/C++ [`WndProc`]s or messages loops in the same process *might* make assumptions about what `id_event`s they'll recieve,
/// and invoke undefined behavior when those assumptions are violated &mdash;
/// perhaps by assuming e.g. a <code>[std::map](https://en.cppreference.com/w/cpp/container/map)&lt;id_event, ...&gt;</code> is populated.
///
/// While I would argue such assumptions are poor form, I cannot make an airtight argument that such assumptions are 100% unsound.
/// Since using this function can undermine such assumptions, it is `unsafe`.
/// There are no other known safety concerns.
///
/// ### Replacing an existing Timer
/// Simply specify the same values to replace that timer.
/// You can ignore the exact value of the returned <code>[Ok]\(_\)</code> in this case.
///
/// ### Creating a new Timer
/// Beware the edge cases!
///
/// If using a **null / [`None`]** HWND, you **should** specify `id_event=0`:
/// *   The timer is uniquely identified by the `hwnd` you passed in, and the **returned** `Ok(id_event)`.
/// *   If you use a non-zero `id_event`, you risk replacing an existing timer, and created timers won't use `id_event`.
/// *   If you don't specify a [`TimerProc`], you'll get no callbacks, and will only see the [`WM::TIMER`] events in message loops.
/// *   [`WM::TIMER`] will be dispatched by message loops on the current thread.
///
/// If using a **non-null** HWND, you "should not" specify `id_event=0`.  Instead, provide your own unique `id_event`:
/// *   The timer is uniquely identified by the `hwnd` and `id_event` you passed in.
/// *   **Ignore** the exact returned value &mdash; `Ok(1)` does not mean `id_event=1`!
/// *   If you specify a zero `id_event`, `id_event` wil be 0 &mdash; `SetTimer` will *not* allocate a unique ID for you.
/// *   If you don't specify a [`TimerProc`], the window's [`WndProc`] will process the [`WM::TIMER`] event instead.
/// *   [`WM::TIMER`] will be dispatched by message loops on the thread that created `hwnd`.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          &mdash; `hwnd` is a valid but unavailable window handle (ex: <code>[get_desktop_window]\(\)</code>)
/// *   [ERROR::INVALID_WINDOW_HANDLE]  &mdash; `hwnd` is an invalid non-null window handle (ex: <code>0x12345678 as HWND</code>)
/// *   [ERROR::INVALID_PARAMETER]      &mdash; `tolerance_delay_ms` is an invalid value.<br>
///     Expected one of:
///     *   `0` = [`TIMERV::DEFAULT_COALESCING`]
///     *   `!0` = `0xFFFFFFFF` = [`TIMERV::NO_COALESCING`]
///     *   `1 ..= 0x7FFFFFF5` (milliseconds)
///
/// ### Examples
/// See [`set_timer`]
///
/// ```
/// # use hwnd::*;
/// # use winresult::ERROR;
/// # unsafe {
/// set_coalescable_timer(None, 0, 0,   Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, !0,  Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), TIMERV::DEFAULT_COALESCING).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), TIMERV::NO_COALESCING     ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), 0x7FFFFFF5                ).unwrap();
/// assert_eq!(set_coalescable_timer(None, 0, 100, Some(timer_func), 0x7FFFFFF6).unwrap_err(), ERROR::INVALID_PARAMETER);
/// assert_eq!(set_coalescable_timer(None, 0, 100, Some(timer_func), 0xFFFFFFFE).unwrap_err(), ERROR::INVALID_PARAMETER);
/// # } // unsafe
///
/// // ...message loop...
/// # unsafe { set_timer(None, 54321, 250, Some(quit)) }.unwrap();
/// # extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
/// # let mut msg = Msg::zeroed();
/// # while get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap() {
/// #   translate_message(&msg);
/// #   let _ = unsafe { dispatch_message_w(&msg) };
/// # }
///
/// extern "system" fn timer_func(_: HWnd, msg: WM32, _: usize, _: u32) {
///     assert_eq!(msg, WM::TIMER);
/// }
/// ```
///
/// ### See Also
/// *   [`set_timerproc_exception_suppression`] - Control exception/panic behavior of [`TimerProc`]s.
/// *   [`set_timer`]                           - Basic alternative defaulting to [`TIMERV::DEFAULT_COALESCING`].
/// *   [`kill_timer`]                          - Cancel or remove the registered timer.
/// *   [`TimerProc`]                           - Callback used if `timer_func` is [`Some`].
/// *   [`WM::TIMER`]                           - Event fired if `timer_func` is [`None`].
pub unsafe fn set_coalescable_timer(hwnd: impl TryInto<HWnd>, id_event: usize, elapse_ms: u32, timer_func: TimerProc, tolerance_delay_ms: u32) -> Result<usize, Error> {
    fn_context!(set_coalescable_timer => SetCoalescableTimer);
    // TODO: tolerance_delay → typed
    let hwnd    = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let tid     = unsafe { SetCoalescableTimer(hwnd, id_event, elapse_ms, core::mem::transmute(timer_func), tolerance_delay_ms) };
    fn_succeeded!(tid != 0)?;
    Ok(tid)
}
