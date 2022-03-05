use crate::*;
use abistr::{TryIntoAsOptCStr, AsOptCStr};
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmdiwindowa)\]
/// CreateMDIWindowA
///
/// Creates a multiple-document interface (mdi) child window.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// let ccs = CreateClientStruct {
///     window_menu:    null_mut(),
///     id_first_child: 0,
/// };
///
/// let mdi_client = unsafe { create_window(abistr::cstr!("MDICLIENT", (),
///     WS::CHILD | WS::CLIPCHILDREN | WS::VSCROLL | WS::HSCROLL,
///     0, 0, 0, 0, null_mut(), null_mut(), None, &css
/// )}.unwrap();
///
/// unsafe {
///
///     let mw = create_mdi_window_a(
///         abistr::cstr!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWnd::MESSAGE, None, 0
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_mdi_window_a<'a>(
    class_name:     impl Into<NameOrAtom<'a, u8>>,
    window_name:    impl TryIntoAsOptCStr,
    style:          impl Into<WindowStyle>,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWnd>,
    hinstance:      impl Into<HInstance<'static>>,
    param:          LPARAM,
) -> Result<HWnd, Error> {
    fn_context!(create_mdi_window_a => CreateMDIWindowA);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?.into();
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateMDIWindowA(
        class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style.into().into(),
        x, y, width, height,
        parent, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd.into())
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmdiwindoww)\]
/// CreateMDIWindowW
///
/// Creates a multiple-document interface (mdi) child window.
///
/// ### Safety
/// The underlying window classes involved may impose soundness requirements on:
/// *   `param`     Might need to be a valid reference to a specific type, might need to be non-null
/// *   `parent`    Might not tolerate children, or the class might not tolerate having a parent
/// *   undocumented global or thread local state
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// unsafe {
///
///     let mw = create_mdi_window_w(
///         abistr::cstr16!("Message"), (), 0,
///         0, 0, 0, 0,
///         HWnd::MESSAGE, None, 0
///     ).unwrap();
///
///     destroy_window(mw).unwrap();
///
/// }
/// ```
pub unsafe fn create_mdi_window_w<'a>(
    class_name:     impl Into<NameOrAtom<'a, u16>>,
    window_name:    impl TryIntoAsOptCStr<u16>,
    style:          impl Into<WindowStyle>,
    x:              i32,
    y:              i32,
    width:          i32,
    height:         i32,
    parent:         impl TryInto<HWnd>,
    hinstance:      impl Into<HInstance<'static>>,
    param:          LPARAM,
) -> Result<HWnd, Error> {
    fn_context!(create_mdi_window_w => CreateMDIWindowW);
    let parent      = parent        .try_into().map_err(|_| fn_param_error!(parent,         ERROR::INVALID_WINDOW_HANDLE))?.into();
    let window_name = window_name   .try_into().map_err(|_| fn_param_error!(window_name,    ERROR::INVALID_WINDOW_HANDLE))?;
    let hwnd = unsafe { CreateMDIWindowW(
        class_name.into().as_atom_or_cstr_ptr(), window_name.as_opt_cstr(), style.into().into(),
        x, y, width, height,
        parent, hinstance.into().into(), param,
    )};
    fn_succeeded!(!hwnd.is_null())?;
    Ok(hwnd.into())
}
