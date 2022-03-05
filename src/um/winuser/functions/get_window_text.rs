use crate::*;
use winapi::um::winuser::*;
use std::ffi::*;
use std::os::raw::c_char;
use std::os::windows::ffi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengtha)\]
/// GetWindowTextLengthA
///
/// Retrieves the length, in characters, of the specified window's title bar text (if the window has a title bar).
/// If the specified window is a control, the function retrieves the length of the text within the control.
/// The docs claim [get_window_text_length_a] cannot retrieve the length of the text of an edit control in another application.
/// However, testing reveals it works fine on at least the desktop window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// let hwnd = unsafe { create_window_ex_a(0, abistr::cstr!("Message"), abistr::cstr!("Title"), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// assert_eq!(Ok(5),                        get_window_text_length_a(hwnd));
/// assert_eq!(Ok(0),                        get_window_text_length_a(get_desktop_window()));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_text_length_a(!42usize as HWND));
/// ```
///
/// ### See Also
/// *   [get_window_text_a]
/// *   [get_window_text_length_w]
pub fn get_window_text_length_a(hwnd: impl TryInto<HWnd>) -> Result<usize, Error> {
    fn_context!(get_window_text_length_a => GetWindowTextLengthA);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    clear_last_error();
    let n = unsafe { GetWindowTextLengthA(hwnd) };
    if n == 0 { fn_error_gle_nz!()? }
    Ok(n as u32 as usize)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthW)\]
/// GetWindowTextLengthW
///
/// Retrieves the length, in characters, of the specified window's title bar text (if the window has a title bar).
/// If the specified window is a control, the function retrieves the length of the text within the control.
/// The docs claim [get_window_text_length_w] cannot retrieve the length of the text of an edit control in another application.
/// However, testing reveals it works fine on at least the desktop window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), abistr::cstr16!("Title"), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// assert_eq!(Ok(5),                        get_window_text_length_w(hwnd));
/// assert_eq!(Ok(0),                        get_window_text_length_w(get_desktop_window()));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_text_length_w(!42usize as HWND));
/// ```
///
/// ### See Also
/// *   [get_window_text_w]
/// *   [get_window_text_length_a]
pub fn get_window_text_length_w(hwnd: impl TryInto<HWnd>) -> Result<usize, Error> {
    fn_context!(get_window_text_length_w => GetWindowTextLengthW);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    clear_last_error();
    let n = unsafe { GetWindowTextLengthW(hwnd) };
    if n == 0 { fn_error_gle_nz!()? }
    Ok(n as u32 as usize)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtexta)\]
/// GetWindowTextA
///
/// Copies the text of the specified window's title bar (if it has one) into a buffer.
/// If the specified window is a control, the text of the control is copied.
/// The docs claim [get_window_text_a] cannot retrieve the text of a control in another application.
/// However, testing reveals it works fine on at least the desktop window.
///
/// ### Errors
/// *   ~~[ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid... *sometimes*.~~
/// *   May succeed with 0 length despite `hwnd` being invalid!
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use abistr::*;
/// # use winresult::*;
/// # use std::os::raw::c_char;
/// # use std::ptr::*;
/// let hwnd = unsafe { create_window_ex_a(0, abistr::cstr!("Message"), abistr::cstr!("Title"), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// let mut title = [0; 1024];
/// let title = get_window_text_a(hwnd, &mut title).unwrap();
/// assert_eq!(b"Title", bytemuck::cast_slice::<c_char, u8>(title));
/// #
/// # assert!(get_window_text_a(get_desktop_window(), title).unwrap().is_empty());
/// # assert!(get_window_text_a(!42usize as HWND,     title).unwrap().is_empty());
/// ```
///
/// ### See Also
/// *   [get_window_text_w]
/// *   [get_window_text_os]
pub fn get_window_text_a(hwnd: impl TryInto<HWnd>, buffer: &mut [c_char]) -> Result<&mut [c_char], Error> {
    fn_context!(get_window_text_a => GetWindowTextA);
    let hwnd    = hwnd  .try_into().map_err(|_| fn_param_error!(hwnd,   ERROR::INVALID_WINDOW_HANDLE))?.into();
    let n : i32 = buffer.len().try_into().unwrap_or(std::i32::MAX);
    clear_last_error(); // read is 0 on error... or on empty string
    let read = unsafe { GetWindowTextA(hwnd, buffer.as_mut_ptr(), n) };
    if n == 0 { fn_error_gle_nz!()? }
    debug_assert!(read <= n);
    Ok(&mut buffer[..read as usize])
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)\]
/// GetWindowTextW
///
/// Copies the text of the specified window's title bar (if it has one) into a buffer.
/// If the specified window is a control, the text of the control is copied.
/// The docs claim [get_window_text_w] cannot retrieve the text of a control in another application.
/// However, testing reveals it works fine on at least the desktop window.
///
/// ### Errors
/// *   ~~[ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid... *sometimes*.~~
/// *   May succeed with 0 length despite `hwnd` being invalid!
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use abistr::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), abistr::cstr16!("Title"), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// let mut title = [0u16; 1024];
/// let title = get_window_text_w(hwnd, &mut title).unwrap();
/// assert_eq!(abistr::cstr16!("Title").to_units(), title);
/// #
/// # assert!(get_window_text_w(get_desktop_window(), title).unwrap().is_empty());
/// # assert!(get_window_text_w(!42usize as HWND,     title).unwrap().is_empty());
/// ```
///
/// ### See Also
/// *   [get_window_text_a]
/// *   [get_window_text_os]
pub fn get_window_text_w(hwnd: impl TryInto<HWnd>, buffer: &mut [u16]) -> Result<&mut [u16], Error> {
    fn_context!(get_window_text_w => GetWindowTextW);
    let hwnd    = hwnd  .try_into().map_err(|_| fn_param_error!(hwnd,   ERROR::INVALID_WINDOW_HANDLE))?.into();
    let n : i32 = buffer.len().try_into().unwrap_or(std::i32::MAX);
    clear_last_error(); // read is 0 on error... or on empty string
    let read = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), n) };
    if n == 0 { fn_error_gle_nz!()? }
    debug_assert!(read <= n);
    Ok(&mut buffer[..read as usize])
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)\]
/// GetWindowTextW
///
/// Copies the text of the specified window's title bar (if it has one) into a buffer.
/// If the specified window is a control, the text of the control is copied.
/// The docs claim [get_window_text_os] cannot retrieve the text of a control in another application.
/// However, testing reveals it works fine on at least the desktop window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use abistr::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), abistr::cstr16!("Title"), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// let mut title = [0u16; 1024];
/// let title = get_window_text_os(hwnd).unwrap();
/// assert_eq!("Title", title.to_string_lossy());
///
/// let title = get_window_text_os(get_desktop_window()).unwrap();
/// assert_eq!("", title.to_string_lossy());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_text_os(!42usize as HWND));
/// ```
///
/// ### See Also
/// *   [get_window_text_a]
/// *   [get_window_text_w]
pub fn get_window_text_os(hwnd: impl TryInto<HWnd>) -> Result<OsString, Error> {
    fn_context!(get_window_text_w => GetWindowTextW);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    let len = get_window_text_length_w(hwnd)?;
    let mut buf = [0u16; 1024];
    if len <= buf.len() {
        Ok(OsString::from_wide(get_window_text_w(hwnd, &mut buf)?))
    } else {
        let mut buf = vec![0u16; len];
        Ok(OsString::from_wide(get_window_text_w(hwnd, &mut buf)?))
    }
}
