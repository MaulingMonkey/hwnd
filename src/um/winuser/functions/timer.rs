use crate::*;
use winapi::shared::minwindef::BOOL;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winuser::*;



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
pub type TimerProcNonNull = unsafe extern "system" fn (hwnd: HWnd, msg: WM32, id_event: usize, tick_count_ms: u32) -> ();

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

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)\]
/// SetTimer
///
/// Creates or replaces a timer that calls [`TimerProc`] every `elapse_ms` milliseconds.
///
/// Each timer is uniquely identified by `(hwnd, id_event)`.
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
///
/// ### Examples
/// ```
/// # use hwnd::*;
/// # use winapi::um::sysinfoapi::GetTickCount;
/// # use winapi::um::winuser::*;
/// # use winresult::ERROR;
/// # use core::cell::Cell;
/// # use core::ptr::null_mut;
/// # let hinstance   = get_module_handle_entry_exe().unwrap();
/// # let name        = abistr::cstr16!("set_timer.docs");
/// # let class       = unsafe { register_class_w(&WndClassW { wnd_proc: Some(wnd_proc), hinstance, class_name: name.into(), .. Default::default() }) }.unwrap();
/// # let hwnd        = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
/// let _ = set_timerproc_exception_suppression(false); // don't suppress timer_func panic!s
///
/// // arbitrary constants - it's up to you to avoid collisions (consider pointer addresses?)
/// const TID_TIMER_FUNC : usize = 42;
/// const TID_WND_PROC   : usize = 9001;
/// thread_local! {
///     static TID_NOWND_TIMER_FUNC : Cell<usize> = const { Cell::new(0) };
///     static TID_NOWND_MSG_LOOP   : Cell<usize> = const { Cell::new(0) };
/// }
///
/// // Each combination of (hwnd, TID_*) creates a unique timer:
/// set_timer(hwnd, TID_TIMER_FUNC, 100, None            ).unwrap();
/// set_timer(hwnd, TID_TIMER_FUNC, 100, Some(timer_func)).unwrap(); // replaces previous line
/// set_timer(hwnd, TID_WND_PROC,   100, None            ).unwrap(); // uses hwnd's wnd_proc
///
/// // Each (None, 0) creates a new unique timer:
/// TID_NOWND_MSG_LOOP  .set(set_timer(None, 0, 100, None).unwrap()); // no callbacks
/// TID_NOWND_TIMER_FUNC.set(set_timer(None, 0, 100, Some(timer_func_nownd)).unwrap());
///
/// // Replacing the above timers:
/// assert_eq!(TID_NOWND_MSG_LOOP.get(),   set_timer(None, TID_NOWND_MSG_LOOP.get(),   100, None).unwrap());
/// assert_eq!(TID_NOWND_TIMER_FUNC.get(), set_timer(None, TID_NOWND_TIMER_FUNC.get(), 100, Some(timer_func_nownd)).unwrap());
///
/// // HWNDs must be None, or a valid, process-local window:
/// assert_eq!(ERROR::ACCESS_DENIED,            set_timer(get_desktop_window(), TID_TIMER_FUNC, 100, Some(timer_func)).unwrap_err(), "set_timer(desktop, ...)");
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE,    set_timer(0x12345678 as HWND,   TID_TIMER_FUNC, 100, Some(timer_func)).unwrap_err(), "set_timer(invalid, ...)");
///
/// // HWNDs need not be thread-local:
/// std::thread::spawn(move || {
///     // will dispatch WM::TIMER s on hwnd's thread
///     // replaces the timers originally registered on the main thread!
///     set_timer(hwnd, TID_TIMER_FUNC, 100, Some(timer_func)).unwrap();
///     set_timer(hwnd, TID_WND_PROC,   100, None            ).unwrap();
///
///     // would dispatch WM::TIMER s on the current thread,
///     // but we never pump a message loop, so these never execute:
///     set_timer(None, 0, 100, None).unwrap();
///     set_timer(None, 0, 100, Some(timer_func_unreachable)).unwrap();
/// }).join().unwrap();
///
/// // ...message loop...
/// # set_timer(None, 54321, 250, Some(quit)).unwrap();
/// # unsafe extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
/// # let mut msg = Msg::zeroed();
/// # let mut timers = 0;
/// # while get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap() {
/// #   if msg.message == WM::TIMER { timers += 1; }
/// #   translate_message(&msg);
/// #   let _ = unsafe { dispatch_message_w(&msg) };
/// # }
/// # assert_eq!(4*2 + 1, timers, "timers"); // each of the 4 timers twice, plus the quit timer once
///
///
///
/// unsafe extern "system" fn timer_func(hwnd: HWnd, msg: WM32, id_event: usize, tick_count: u32) {
///     assert!(!hwnd.is_null(),                                "timer_func: hwnd");
///     assert_eq!(msg,         WM::TIMER,                      "timer_func: msg");
///     assert_eq!(id_event,    TID_TIMER_FUNC,                 "timer_func: id_event");
///     assert!(GetTickCount().wrapping_sub(tick_count) < 1000, "timer_func: tick_count");
///     // not entirely sure if tick_count is when WM::TIMER was enqueued, dequeued, or ...?
/// }
///
/// unsafe extern "system" fn timer_func_nownd(hwnd: HWnd, msg: WM32, id_event: usize, tick_count: u32) {
///     assert!(hwnd.is_null(),                                 "timer_func_nownd: hwnd");
///     assert_eq!(msg,         WM::TIMER,                      "timer_func_nownd: msg");
///     assert_eq!(id_event,    TID_NOWND_TIMER_FUNC.get(),     "timer_func_nownd: id_event");
///     assert!(GetTickCount().wrapping_sub(tick_count) < 1000, "timer_func_nownd: tick_count");
/// }
///
/// unsafe extern "system" fn timer_func_unreachable(_: HWnd, _: WM32, _: usize, _: u32) {
///     panic!("timer_func_unreachable: never expected to be executed");
/// }
///
/// unsafe extern "system" fn wnd_proc(hwnd: HWnd, msg: WM32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
///     match msg {
///         WM::TIMER => {
///             // N.B. wnd_proc is only called if timer_proc is None and hwnd isn't
///             assert!(!hwnd.is_null());
///             assert_eq!(msg,    WM::TIMER    );
///             assert_eq!(wparam, TID_WND_PROC );
///             assert_eq!(lparam, 0            ); // ≈ TimerProc
///         },
///         _ => {},
///     }
///     unsafe { def_window_proc_w(hwnd, msg, wparam, lparam) }
/// }
/// ```
///
/// ### See Also
/// *   [`set_timerproc_exception_suppression`] - Control exception/panic behavior of [`TimerProc`]s.
/// *   [`set_coalescable_timer`]               - Alternative giving more control over how events are coalesced.
/// *   [`kill_timer`]                          - Cancel or remove the registered timer.
/// *   [`TimerProc`]                           - Callback used if `timer_func` is [`Some`].
/// *   [`WM::TIMER`]                           - Event fired if `timer_func` is [`None`].
pub fn set_timer(hwnd: impl TryInto<HWnd>, id_event: usize, elapse_ms: u32, timer_func: TimerProc) -> Result<usize, Error> {
    fn_context!(set_timer => SetTimer);
    let hwnd    = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let tid     = unsafe { SetTimer(hwnd, id_event, elapse_ms, core::mem::transmute(timer_func)) };
    fn_succeeded!(tid != 0)?;
    Ok(tid)
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcoalescabletimer)\]
/// SetCoalescableTimer
///
/// Creates or replaces a timer that calls [`TimerProc`] every `elapse_ms` milliseconds.
///
/// Each timer is uniquely identified by `(hwnd, id_event)`.
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
/// set_coalescable_timer(None, 0, 0,   Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, !0,  Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), 0                         ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), TIMERV::DEFAULT_COALESCING).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), TIMERV::NO_COALESCING     ).unwrap();
/// set_coalescable_timer(None, 0, 100, Some(timer_func), 0x7FFFFFF5                ).unwrap();
/// assert_eq!(set_coalescable_timer(None, 0, 100, Some(timer_func), 0x7FFFFFF6).unwrap_err(), ERROR::INVALID_PARAMETER);
/// assert_eq!(set_coalescable_timer(None, 0, 100, Some(timer_func), 0xFFFFFFFE).unwrap_err(), ERROR::INVALID_PARAMETER);
///
/// // ...message loop...
/// # set_timer(None, 54321, 250, Some(quit)).unwrap();
/// # unsafe extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
/// # let mut msg = Msg::zeroed();
/// # while get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap() {
/// #   translate_message(&msg);
/// #   let _ = unsafe { dispatch_message_w(&msg) };
/// # }
///
/// unsafe extern "system" fn timer_func(_: HWnd, msg: WM32, _: usize, _: u32) {
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
pub fn set_coalescable_timer(hwnd: impl TryInto<HWnd>, id_event: usize, elapse_ms: u32, timer_func: TimerProc, tolerance_delay_ms: u32) -> Result<usize, Error> {
    fn_context!(set_coalescable_timer => SetCoalescableTimer);
    // TODO: tolerance_delay → typed
    let hwnd    = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let tid     = unsafe { SetCoalescableTimer(hwnd, id_event, elapse_ms, core::mem::transmute(timer_func), tolerance_delay_ms) };
    fn_succeeded!(tid != 0)?;
    Ok(tid)
}

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
/// # let name        = abistr::cstr16!("kill_timer.docs");
/// # let class       = unsafe { register_class_w(&WndClassW { wnd_proc: Some(def_window_proc_w), hinstance, class_name: name.into(), .. Default::default() }) }.unwrap();
/// # let hwnd1       = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
/// # let hwnd2       = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
/// # let desktop     = get_desktop_window();
/// # let invalid     = HWnd::from(0x12345678 as HWND);
/// let _ = set_timerproc_exception_suppression(false); // don't suppress timer_func panic!s
///
/// // Each of these is a unique timer:
/// set_timer(None,  42, 100, Some(timer_func)).expect("None");  // (None,  42)
/// set_timer(hwnd1, 42, 100, Some(timer_func)).expect("hwnd1"); // (hwnd1, 42)
/// set_timer(hwnd2, 42, 100, Some(timer_func)).expect("hwnd2"); // (hwnd2, 42)
///
/// // triggered 3x after 1/10th of a second (100 milliseconds):
/// unsafe extern "system" fn timer_func(hwnd: HWnd, _: WM32, id_event: usize, _: u32) {
///     # TIMER_FUNC_CALLS.fetch_add(1, Ordering::Relaxed);
///     kill_timer(hwnd, id_event).unwrap(); // fire each timer only once
///     kill_timer(hwnd, id_event).unwrap_err(); // already killed
/// }
///
/// // ...message loop...
/// # set_timer(None, 42, 300, Some(quit)).unwrap();
/// # unsafe extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
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

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)\]
/// SetUserObjectInformationW(GetCurrentProcess(), UOI_TIMERPROC_EXCEPTION_SUPPRESSION, ...)
///
/// Controls wheither [`TimerProc`]s are wrapped in an exception handler that suppresses/discards all exceptions
/// (`true` &mdash; default as of Windows 2000, although this might change in the future)
/// or if said exceptions can percolate out of [`get_message_w`] etc. presumably triggering debuggers, crash handlers, etc.
/// (`false` &mdash; **recommended by Microsoft for predictability and security purposes**.)
///
/// ### Errors
/// Unlikely to fail on modern Windows in practice.  This could *theoretically* fail if:
/// *   Configuring TimerProc suppression is disabled by a new policy, security flags, debugger, hooking/patching functions, etc.
/// *   Running on pre-NT Windows, WINE, ReactOS, or other non-Windows system (with stubbed `SetUserObjectInformationW`?)
///
/// ### Examples
/// ```
/// # use hwnd::*;
/// #
/// // As recommended by Microsoft for predictability and security purposes
/// // (call before `set_timer` etc.)
/// let _ = set_timerproc_exception_suppression(false);
/// # set_timerproc_exception_suppression(false).unwrap();
///
/// // Restore default behavior, not recommended:
/// let _ = set_timerproc_exception_suppression(true);
/// # set_timerproc_exception_suppression(true).unwrap();
/// ```
///
/// ### See Also
/// *   [`set_timer`]
/// *   [`set_coalescable_timer`]
/// *   [`TimerProc`]
/// *   [`TimerProcNonNull`]
///
/// ### `SetTimer` Remarks
///
/// Before using `SetTimer` or other timer-related functions, it is recommended to set the `UOI_TIMERPROC_EXCEPTION_SUPPRESSION` flag to `false` through the `SetUserObjectInformationW` function, otherwise the application could behave unpredictably and could be vulnerable to security exploits. For more info, see [SetUserObjectInformationW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw).
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer#remarks>
///
/// ### `SetUserObjectInformationW` Parameters
///
/// If TRUE, Windows will enclose its calls to [TimerProc](https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nc-winuser-timerproc) with an exception handler that consumes and discards all exceptions. This has been the default behavior since Windows 2000, although that may change in future versions of Windows.
///
/// If \[...\] FALSE, Windows will not enclose its calls to [TimerProc](https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nc-winuser-timerproc) with an exception handler. A setting of FALSE is recommended. Otherwise, the application could behave unpredictably, and could be more vulnerable to security exploits.
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw#parameters>
pub fn set_timerproc_exception_suppression(suppress: bool) -> Result<(), Error> {
    fn_context!(set_timerproc_exception_suppression => SetUserObjectInformationW);
    let mut suppress : BOOL = if suppress { 1 } else { 0 };
    let suppress : *mut BOOL = &mut suppress;
    fn_succeeded!(unsafe { SetUserObjectInformationW(GetCurrentProcess(), UOI_TIMERPROC_EXCEPTION_SUPPRESSION as _, suppress.cast(), size_of_32::<BOOL>()) })
}
