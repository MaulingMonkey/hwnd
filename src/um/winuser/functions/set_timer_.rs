use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)\]
/// SetTimer
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
/// # unsafe {
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
/// # } // unsafe
///
/// // ...message loop...
/// # unsafe { set_timer(None, 54321, 250, Some(quit)) }.unwrap();
/// # extern "system" fn quit(_: HWnd, _: WM32, _: usize, _: u32) { post_quit_message(0) }
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
/// extern "system" fn timer_func(hwnd: HWnd, msg: WM32, id_event: usize, tick_count: u32) {
///     assert!(!hwnd.is_null(),                                    "timer_func: hwnd");
///     assert_eq!(msg,         WM::TIMER,                          "timer_func: msg");
///     assert_eq!(id_event,    TID_TIMER_FUNC,                     "timer_func: id_event");
///     assert!(get_tick_count().wrapping_sub(tick_count) < 1000,   "timer_func: tick_count");
///     // not entirely sure if tick_count is when WM::TIMER was enqueued, dequeued, or ...?
/// }
///
/// extern "system" fn timer_func_nownd(hwnd: HWnd, msg: WM32, id_event: usize, tick_count: u32) {
///     assert!(hwnd.is_null(),                                     "timer_func_nownd: hwnd");
///     assert_eq!(msg,         WM::TIMER,                          "timer_func_nownd: msg");
///     assert_eq!(id_event,    TID_NOWND_TIMER_FUNC.get(),         "timer_func_nownd: id_event");
///     assert!(get_tick_count().wrapping_sub(tick_count) < 1000,   "timer_func_nownd: tick_count");
/// }
///
/// extern "system" fn timer_func_unreachable(_: HWnd, _: WM32, _: usize, _: u32) {
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
///
/// fn get_tick_count() -> u32 {
///     unsafe { GetTickCount() }
/// }
/// ```
///
/// ### See Also
/// *   [`set_timerproc_exception_suppression`] - Control exception/panic behavior of [`TimerProc`]s.
/// *   [`set_coalescable_timer`]               - Alternative giving more control over how events are coalesced.
/// *   [`kill_timer`]                          - Cancel or remove the registered timer.
/// *   [`TimerProc`]                           - Callback used if `timer_func` is [`Some`].
/// *   [`WM::TIMER`]                           - Event fired if `timer_func` is [`None`].
pub unsafe fn set_timer(hwnd: impl TryInto<HWnd>, id_event: usize, elapse_ms: u32, timer_func: TimerProc) -> Result<usize, Error> {
    fn_context!(set_timer => SetTimer);
    let hwnd    = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let tid     = unsafe { SetTimer(hwnd, id_event, elapse_ms, core::mem::transmute(timer_func)) };
    fn_succeeded!(tid != 0)?;
    Ok(tid)
}
