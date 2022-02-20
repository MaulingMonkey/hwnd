use crate::*;
use crate::PM::PeekMessageFlags;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagea)\]
/// PeekMessageA
///
/// Dispatches incoming nonqueued messages, checks the thread message queue for a posted message, and retrieves the message (if any exist).
///
/// ### Edge cases
/// *   If `min` = `max` = `0`, no filtering of WM::\* occurs.
/// *   If `min` > `max`, **may panic (debug) or fail to process [WM::QUIT] (release)!**
/// *   If `hwnd` belongs to another thread,  this will silently return [None] (or Some(Msg { message: WM::QUIT, ... }))
/// *   If `hwnd` belongs to another process, this will silently return [None] (or Some(Msg { message: WM::QUIT, ... }))
/// *   If `hwnd` belongs to this thread, only messages for that window will be retrieved.
/// *   If `hwnd` is [HWnd::NULL], messages for all windows of the current thread, and thread messages, are retrieved.
/// *   If `hwnd` is `-1isize as HWND`, only thread messages are retrieved (e.g. for which `msg.hwnd` is [HWnd::NULL])
///
/// ### Returns
/// *   None        if no matching messages are available
/// *   Some([Msg]) if a matching message is peeked or removed
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
///     // ...register classes, create windows, etc...
///
///     loop {
///         while let Some(msg) = peek_message_a(HWnd::NULL, 0, 0, PM::REMOVE) {
///             if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///             translate_message(&msg);
///             let _ = unsafe { dispatch_message_a(&msg) };
///         }
///
///         // ...render...
///     }
/// }
/// ```
#[must_use] pub fn peek_message_a(hwnd: impl Into<HWnd>, min: impl Into<WM32>, max: impl Into<WM32>, remove_msg: impl Into<PeekMessageFlags>) -> Option<Msg> {
    fn_context!(peek_message_a => PeekMessageA);
    let hwnd = hwnd.into().into();
    let min : u32 = min.into().into();
    let max : u32 = max.into().into();
    let remove_msg = remove_msg.into().into();

    debug_assert!(min <= max, "min ({min}) > max ({max}) may cause WM_QUIT to not be processed");

    let mut msg = Msg::zeroed();
    match unsafe { PeekMessageA(msg.as_mut(), hwnd, min, max, remove_msg) } {
        0   => None,
        1|_ => Some(msg),
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)\]
/// PeekMessageW
///
/// Dispatches incoming nonqueued messages, checks the thread message queue for a posted message, and retrieves the message (if any exist).
///
/// ### Edge cases
/// *   If `min` = `max` = `0`, no filtering of WM::\* occurs.
/// *   If `min` > `max`, **may panic (debug) or fail to process [WM::QUIT] (release)!**
/// *   If `hwnd` belongs to another thread,  this will silently return [None] (or Some(Msg { message: WM::QUIT, ... }))
/// *   If `hwnd` belongs to another process, this will silently return [None] (or Some(Msg { message: WM::QUIT, ... }))
/// *   If `hwnd` belongs to this thread, only messages for that window will be retrieved.
/// *   If `hwnd` is [HWnd::NULL], messages for all windows of the current thread, and thread messages, are retrieved.
/// *   If `hwnd` is `-1isize as HWND`, only thread messages are retrieved (e.g. for which `msg.hwnd` is [HWnd::NULL])
///
/// ### Returns
/// *   None        if no matching messages are available
/// *   Some([Msg]) if a matching message is peeked or removed
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
///     // ...register classes, create windows, etc...
///
///     loop {
///         while let Some(msg) = peek_message_w(HWnd::NULL, 0, 0, PM::REMOVE) {
///             if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///             translate_message(&msg);
///             let _ = unsafe { dispatch_message_w(&msg) };
///         }
///
///         // ...render...
///     }
/// }
/// ```
#[must_use] pub fn peek_message_w(hwnd: impl Into<HWnd>, min: impl Into<WM32>, max: impl Into<WM32>, remove_msg: impl Into<PeekMessageFlags>) -> Option<Msg> {
    fn_context!(peek_message_w => PeekMessageW);
    let hwnd = hwnd.into().into();
    let min : u32 = min.into().into();
    let max : u32 = max.into().into();
    let remove_msg = remove_msg.into().into();

    debug_assert!(min <= max, "min ({min}) > max ({max}) may cause WM_QUIT to not be processed");

    let mut msg = Msg::zeroed();
    match unsafe { PeekMessageW(msg.as_mut(), hwnd, min, max, remove_msg) } {
        0   => None,
        1|_ => Some(msg),
    }
}
