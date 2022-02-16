use crate::*;
use abistr::{TryIntoAsOptCStr, AsOptCStr};
use winapi::um::winuser::*;
use std::ffi::c_void;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa)\]
/// CreateWindowA
///
/// Creates an overlapped, pop-up, or child window.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   `hmenu`     Might need to be valid
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// unsafe {
///
///     let mw = create_window_a(
///         abistr::cstr!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWND_MESSAGE, null_mut(), None, null_mut()
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_window_a<'a>(
    class_name:     impl Into<NameOrAtom<'a, u8>>,
    window_name:    impl TryIntoAsOptCStr,
    style:          WS,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWND>,
    hmenu:          HMENU,
    hinstance:      impl Into<HInstance<'static>>,
    param:          *mut c_void,
) -> Result<HWND, Error> {
    fn_context!(create_window_a => CreateWindowA);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?;
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateWindowExA(
        0, class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style,
        x, y, width, height,
        parent, hmenu, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)\]
/// CreateWindowW
///
/// Creates an overlapped, pop-up, or child window.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   `hmenu`     Might need to be valid
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// unsafe {
///
///     let mw = create_window_w(
///         abistr::cstr16!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWND_MESSAGE, null_mut(), None, null_mut()
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_window_w<'a>(
    class_name:     impl Into<NameOrAtom<'a, u16>>,
    window_name:    impl TryIntoAsOptCStr<u16>,
    style:          WS,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWND>,
    hmenu:          HMENU,
    hinstance:      impl Into<HInstance<'static>>,
    param:          *mut c_void,
) -> Result<HWND, Error> {
    fn_context!(create_window_w => CreateWindowW);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?;
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateWindowExW(
        0, class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style,
        x, y, width, height,
        parent, hmenu, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa)\]
/// CreateWindowExA
///
/// Creates an overlapped, pop-up, or child window with an extended window style.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   `hmenu`     Might need to be valid
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// unsafe {
///
///     let mw = create_window_ex_a(
///         0, abistr::cstr!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWND_MESSAGE, null_mut(), None, null_mut()
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_window_ex_a<'a>(
    ex_style:       WS_EX,
    class_name:     impl Into<NameOrAtom<'a, u8>>,
    window_name:    impl TryIntoAsOptCStr,
    style:          WS,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWND>,
    hmenu:          HMENU,
    hinstance:      impl Into<HInstance<'static>>,
    param:          *mut c_void,
) -> Result<HWND, Error> {
    fn_context!(create_window_ex_a => CreateWindowExA);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?;
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateWindowExA(
        ex_style, class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style,
        x, y, width, height,
        parent, hmenu, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)\]
/// CreateWindowExW
///
/// Creates an overlapped, pop-up, or child window with an extended window style.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   `hmenu`     Might need to be valid
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// unsafe {
///
///     let mw = create_window_ex_w(
///         0, abistr::cstr16!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWND_MESSAGE, null_mut(), None, null_mut()
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_window_ex_w<'a>(
    ex_style:       WS_EX,
    class_name:     impl Into<NameOrAtom<'a, u16>>,
    window_name:    impl TryIntoAsOptCStr<u16>,
    style:          WS,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWND>,
    hmenu:          HMENU,
    hinstance:      impl Into<HInstance<'static>>,
    param:          *mut c_void,
) -> Result<HWND, Error> {
    fn_context!(create_window_ex_w => CreateWindowExW);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?;
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateWindowExW(
        ex_style, class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style,
        x, y, width, height,
        parent, hmenu, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd)
}
