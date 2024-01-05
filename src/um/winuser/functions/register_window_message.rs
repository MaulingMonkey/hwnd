use crate::*;
use abistr::AsCStr;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerwindowmessagea)\]
/// RegisterWindowMessageA
///
/// Defines a new window message that is guaranteed to be unique throughout the system.
/// The message value can be used when sending or posting messages.
///
/// Only use [register_window_message_a] when more than one application must process the same message.
/// For sending private messages within a window class, an application can use any integer in the range WM::USER through 0x7FFF.
///
/// ### Errors
/// *   [ERROR::INVALID_PARAMETER]  if `string` cannot be converted to a valid c string
/// *   [ERROR::NOT_ENOUGH_MEMORY]  if the system is unable to allocate a unique message constant between 0xC000 and 0xFFFF
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// let wm1 = register_window_message_a(abistr::cstr!("WM_EXAMPLE")).unwrap();
/// let wm2 = register_window_message_a(abistr::cstr!("WM_EXAMPLE")).unwrap();
/// assert_eq!(wm1, wm2);
///
/// // ...leak tons of IDs...
/// # if !cfg!(nope) { return } // don't run the system out of atoms
/// # for i in 0xC000 .. 0xFFFF {
/// #   let wm_name = format!("WM_EXAMPLE_{i}\0");
/// #   let wm_name = abistr::CStrNonNull::from_units_with_nul(wm_name.as_bytes()).unwrap();
/// #   let _ = register_window_message_a(wm_name);
/// # }
///
/// assert_eq!(ERROR::NOT_ENOUGH_MEMORY, register_window_message_a(abistr::cstr!("WM_EXAMPLE2")).unwrap_err());
/// ```
pub fn register_window_message_a(string: impl abistr::TryIntoAsCStr) -> Result<WM32, Error> {
    fn_context!(register_window_message_a => RegisterWindowMessageA);
    let string = string.try_into().map_err(|_| fn_param_error!(string, ERROR::INVALID_PARAMETER))?;
    let string = string.as_cstr();
    let m = unsafe { RegisterWindowMessageA(string) };
    fn_succeeded!(m != 0)?;
    Ok(WM32::from(m))
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerwindowmessagew)\]
/// RegisterWindowMessageW
///
/// Defines a new window message that is guaranteed to be unique throughout the system.
/// The message value can be used when sending or posting messages.
///
/// Only use [register_window_message_w] when more than one application must process the same message.
/// For sending private messages within a window class, an application can use any integer in the range WM::USER through 0x7FFF.
///
/// ### Errors
/// *   [ERROR::INVALID_PARAMETER]  if `string` cannot be converted to a valid c string
/// *   [ERROR::NOT_ENOUGH_MEMORY]  if the system is unable to allocate a unique message constant between 0xC000 and 0xFFFF
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// let wm1 = register_window_message_w(abistr::cstr16!("WM_EXAMPLE")).unwrap();
/// let wm2 = register_window_message_w(abistr::cstr16!("WM_EXAMPLE")).unwrap();
/// assert_eq!(wm1, wm2);
///
/// // ...leak tons of IDs...
/// # if !cfg!(nope) { return } // don't run the system out of atoms
/// # for i in 0xC000 .. 0xFFFF {
/// #   let wm_name = format!("WM_EXAMPLE_{i}\0").encode_utf16().collect::<Vec<_>>();
/// #   let wm_name = abistr::CStrNonNull::<u16>::from_units_with_nul(&wm_name).unwrap();
/// #   let _ = register_window_message_w(wm_name);
/// # }
///
/// assert_eq!(ERROR::NOT_ENOUGH_MEMORY, register_window_message_w(abistr::cstr16!("WM_EXAMPLE2")).unwrap_err());
/// ```
pub fn register_window_message_w(string: impl abistr::TryIntoAsCStr<u16>) -> Result<WM32, Error> {
    fn_context!(register_window_message_w => RegisterWindowMessageW);
    let string = string.try_into().map_err(|_| fn_param_error!(string, ERROR::INVALID_PARAMETER))?;
    let string = string.as_cstr();
    let m = unsafe { RegisterWindowMessageW(string) };
    fn_succeeded!(m != 0)?;
    Ok(WM32::from(m))
}
