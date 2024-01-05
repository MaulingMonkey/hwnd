use crate::*;
use winapi::um::winuser::*;



// TODO: get_translate_dispatch_message ?

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagea)\]
/// GetMessageA
///
/// Get a message from the current thread's message queue, blocking if the queue is empty.
///
/// ### Edge cases
/// *   If `min` = `max` = `0`, no filtering of WM::\* occurs.
/// *   If `min` > `max`, **may panic (debug) or fail to process [WM::QUIT] (release)!**
/// *   If `hwnd` belongs to another thread, **may panic (debug) or hang (release)!**
/// *   If `hwnd` belongs to another process, **may panic (debug) or hang (release)!**
/// *   If `hwnd` belongs to this thread, only messages for that window will be retrieved.
/// *   If `hwnd` is [HWnd::NULL], messages for all windows of the current thread, and thread messages, are retrieved.
/// *   If `hwnd` is `-1isize as HWND`, only thread messages are retrieved (e.g. for which `msg.hwnd` is [HWnd::NULL])
///
/// ### Returns
/// *   Ok(false)   on [WM::QUIT]
/// *   Ok(true)    on most other messages
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]              If `hwnd` is not [`HWnd::NULL`], `-1 as HWND`, or a valid `hwnd`
/// *   [ERROR::ACCESS_DENIED]                      Future process check of hwnd?
/// *   [ERROR::WINDOW_OF_OTHER_THREAD]             Future thread check of hwnd?
///
/// ### Example
/// ```
/// use hwnd::*;
///
/// fn main() {
/// #   post_quit_message(0); // don't hang test
/// #
/// #   let mut msg = Msg::zeroed();
/// #   assert_eq!(winresult::ERROR::INVALID_WINDOW_HANDLE, get_message_a(&mut msg, !42usize as HWND, 0, 0).unwrap_err());
///     // ...
///
///     loop {
///         let mut msg = Msg::zeroed();
///         get_message_a(&mut msg, HWnd::NULL, 0, 0).unwrap();
///
///         if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///
///         translate_message(&msg);
///         let _ = unsafe { dispatch_message_a(&msg) };
///     }
/// }
/// ```
pub fn get_message_a(msg: &mut impl AsMut<Msg>, hwnd: impl Into<HWnd>, min: impl Into<WM32>, max: impl Into<WM32>) -> Result<bool, Error> {
    fn_context!(get_message_a => GetMessageA);
    let msg = msg.as_mut().as_mut();
    let hwnd = hwnd.into();
    let min : u32 = min.into().into();
    let max : u32 = max.into().into();

    if cfg!(debug_assertions) {
        assert!(min <= max, "min ({min}) > max ({max}) may cause WM_QUIT to not be processed");
        if let Ok((t, p)) = get_window_thread_process_id(hwnd) {
            // XXX: I could convert these to error codes instead?
            assert_eq!(p, get_current_process_id(), "GetMessageA may hang if fed a hwnd belonging to another process");
            assert_eq!(t, get_current_thread_id(),  "GetMessageA may hang if fed a hwnd belonging to another thread");
        }
    }

    match unsafe { GetMessageA(msg, hwnd.into(), min, max) } {
        1       => Ok(true),
        0       => Ok(false),               // WM::QUIT
        -1 | _  => Err(fn_error_gle!()),    // Error
    }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)\]
/// GetMessageW
///
/// Get a message from the current thread's message queue, blocking if the queue is empty.
///
/// ### Edge cases
/// *   If `min` = `max` = `0`, no filtering of WM::\* occurs.
/// *   If `min` > `max`, **may panic (debug) or fail to process [WM::QUIT] (release)!**
/// *   If `hwnd` belongs to another thread, **may panic (debug) or hang (release)!**
/// *   If `hwnd` belongs to another process, **may panic (debug) or hang (release)!**
/// *   If `hwnd` belongs to this thread, only messages for that window will be retrieved.
/// *   If `hwnd` is [HWnd::NULL], messages for all windows of the current thread, and thread messages, are retrieved.
/// *   If `hwnd` is `-1isize as HWND`, only thread messages are retrieved (e.g. for which `msg.hwnd` is [HWnd::NULL])
///
/// ### Returns
/// *   Ok(false)   on [WM::QUIT]
/// *   Ok(true)    on most other messages
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]              If `hwnd` is not [`HWnd::NULL`], `-1 as HWND`, or a valid `hwnd`
/// *   [ERROR::ACCESS_DENIED]                      Future process check of hwnd?
/// *   [ERROR::WINDOW_OF_OTHER_THREAD]             Future thread check of hwnd?
///
/// ### Example
/// ```
/// use hwnd::*;
///
/// fn main() {
/// #   post_quit_message(0); // don't hang test
/// #
/// #   let mut msg = Msg::zeroed();
/// #   assert_eq!(winresult::ERROR::INVALID_WINDOW_HANDLE, get_message_w(&mut msg, !42usize as HWND, 0, 0).unwrap_err());
///     // ...
///
///     loop {
///         let mut msg = Msg::zeroed();
///         get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap();
///
///         if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///
///         translate_message(&msg);
///         let _ = unsafe { dispatch_message_w(&msg) };
///     }
/// }
/// ```
pub fn get_message_w(msg: &mut impl AsMut<Msg>, hwnd: impl Into<HWnd>, min: impl Into<WM32>, max: impl Into<WM32>) -> Result<bool, Error> {
    fn_context!(get_message_w => GetMessageW);
    let msg = msg.as_mut().as_mut();
    let hwnd = hwnd.into();
    let min : u32 = min.into().into();
    let max : u32 = max.into().into();

    if cfg!(debug_assertions) {
        assert!(min <= max, "min ({min}) > max ({max}) may cause WM_QUIT to not be processed");
        if let Ok((t, p)) = get_window_thread_process_id(hwnd) {
            // XXX: I could convert these to error codes instead?
            assert_eq!(p, get_current_process_id(), "GetMessageW may hang if fed a hwnd belonging to another process");
            assert_eq!(t, get_current_thread_id(),  "GetMessageW may hang if fed a hwnd belonging to another thread");
        }
    }

    match unsafe { GetMessageW(msg, hwnd.into(), min, max) } {
        1       => Ok(true),
        0       => Ok(false),               // WM::QUIT
        -1 | _  => Err(fn_error_gle!()),    // Error
    }
}



#[cfg(debug_assertions)] #[test] #[should_panic] fn get_message_a_wrong_process() {
    let mut msg = Msg::zeroed();
    let _ = get_message_a(&mut msg, get_desktop_window(), 0, 0);
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn get_message_a_wrong_thread() {
    use std::ptr::*;
    let (s, r) = std::sync::mpsc::channel();
    let t = std::thread::spawn(move || {
        s.send(unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() }).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(3));
    });
    let hwnd = r.recv().unwrap();
    let mut msg = Msg::zeroed();
    let _ = get_message_a(&mut msg, hwnd, 0, 0);
    t.join().unwrap();
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn get_message_w_wrong_process() {
    let mut msg = Msg::zeroed();
    let _ = get_message_w(&mut msg, get_desktop_window(), 0, 0);
}

#[cfg(debug_assertions)] #[test] #[should_panic] fn get_message_w_wrong_thread() {
    use std::ptr::*;
    let (s, r) = std::sync::mpsc::channel();
    let t = std::thread::spawn(move || {
        s.send(unsafe { create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() }).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(3));
    });
    let hwnd = r.recv().unwrap();
    let mut msg = Msg::zeroed();
    let _ = get_message_w(&mut msg, hwnd, 0, 0);
    t.join().unwrap();
}
