use crate::*;

use winapi::shared::minwindef::BOOL;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrect)\]
/// AdjustWindowRect
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      **In:** The desired client area.  **Out:** the required outer area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let mut area = RECT { left: 0, right: 800, top: 0, bottom: 600 };
/// adjust_window_rect(&mut area, WS_OVERLAPPEDWINDOW, false).unwrap();
///
/// let mut area = RECT { left: 0, right: -800, top: 0, bottom: 600 };
/// adjust_window_rect(&mut area, WS_OVERLAPPEDWINDOW, false).unwrap();
/// ```
pub fn adjust_window_rect(rect: &mut impl AsMut<RECT>, style: WS, menu: impl Into<bool>) -> Result<(), Error> {
    fn_context!(adjust_window_rect => AdjustWindowRect);
    fn_succeeded!(unsafe { AdjustWindowRect(rect.as_mut().as_mut(), style, menu.into() as BOOL) });
    Ok(())
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrect)\]
/// AdjustWindowRect
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      The desired client area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
///
/// ### Returns
/// *   The required outer window area.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let area = adjust_window_rect_copy(RECT { left: 0, right: 800, top: 0, bottom: 600 }, WS_OVERLAPPEDWINDOW, false).unwrap();
/// let area = adjust_window_rect_copy(RECT { left: 0, right:-800, top: 0, bottom: 600 }, WS_OVERLAPPEDWINDOW, false).unwrap();
/// ```
pub fn adjust_window_rect_copy(rect: impl Into<RECT>, style: WS, menu: impl Into<bool>) -> Result<RECT, Error> {
    let mut rect = rect.into();
    adjust_window_rect(&mut rect, style, menu)?;
    Ok(rect)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)\]
/// AdjustWindowRectEx
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      **In:** The desired client area.  **Out:** the required outer area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
/// *   `ex_style`  The [extended window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles) of the window in question.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let mut area = RECT { left: 0, right: 800, top: 0, bottom: 600 };
/// adjust_window_rect_ex(&mut area, WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW).unwrap();
///
/// let mut area = RECT { left: 0, right: -800, top: 0, bottom: 600 };
/// adjust_window_rect_ex(&mut area, WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW).unwrap();
/// ```
pub fn adjust_window_rect_ex(rect: &mut impl AsMut<RECT>, style: WS, menu: impl Into<bool>, ex_style: WS_EX) -> Result<(), Error> {
    fn_context!(adjust_window_rect_ex => AdjustWindowRectEx);
    fn_succeeded!(unsafe { AdjustWindowRectEx(rect.as_mut().as_mut(), style, menu.into() as BOOL, ex_style) });
    Ok(())
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)\]
/// AdjustWindowRectEx
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      The desired client area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
/// *   `ex_style`  The [extended window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles) of the window in question.
///
/// ### Returns
/// *   The required outer window area.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let area = adjust_window_rect_ex_copy(
///     RECT { left: 0, right: 800, top: 0, bottom: 600 },
///     WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW,
/// ).unwrap();
///
/// let area = adjust_window_rect_ex_copy(
///     RECT { left: 0, right:-800, top: 0, bottom: 600 },
///     WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW,
/// ).unwrap();
/// ```
pub fn adjust_window_rect_ex_copy(rect: impl Into<RECT>, style: WS, menu: impl Into<bool>, ex_style: WS_EX) -> Result<RECT, Error> {
    let mut rect = rect.into();
    adjust_window_rect_ex(&mut rect, style, menu, ex_style)?;
    Ok(rect)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectexfordpi)\]
/// AdjustWindowRectExForDpi
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      **In:** The desired client area.  **Out:** the required outer area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
/// *   `ex_style`  The [extended window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles) of the window in question.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let mut area = RECT { left: 0, right: 800, top: 0, bottom: 600 };
/// adjust_window_rect_ex_for_dpi(&mut area, WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW, 100).unwrap();
///
/// let mut area = RECT { left: 0, right: -800, top: 0, bottom: 600 };
/// adjust_window_rect_ex_for_dpi(&mut area, WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW, 100).unwrap();
/// ```
pub fn adjust_window_rect_ex_for_dpi(rect: &mut impl AsMut<RECT>, style: WS, menu: impl Into<bool>, ex_style: WS_EX, dpi: u32) -> Result<(), Error> {
    fn_context!(adjust_window_rect_ex_for_dpi => AdjustWindowRectExForDpi);
    fn_succeeded!(unsafe { AdjustWindowRectExForDpi(rect.as_mut().as_mut(), style, menu.into() as BOOL, ex_style, dpi) });
    Ok(())
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectexfordpi)\]
/// AdjustWindowRectExForDpi
///
/// Calculates the outer window size required for a given client area.
///
/// ### Arguments
/// *   `rect`      The desired client area.
/// *   `style`     The [window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles) of the window in question.
/// *   `menu`      Use `true` if the window in question has a menu, `false` otherwise.
/// *   `ex_style`  The [extended window style](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles) of the window in question.
///
/// ### Returns
/// *   The required outer window area.
///
/// ### Errors
/// *   ???
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let area = adjust_window_rect_ex_for_dpi_copy(
///     RECT { left: 0, right: 800, top: 0, bottom: 600 },
///     WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW, 100,
/// ).unwrap();
///
/// let area = adjust_window_rect_ex_for_dpi_copy(
///     RECT { left: 0, right:-800, top: 0, bottom: 600 },
///     WS_OVERLAPPEDWINDOW, false, WS_EX_TOOLWINDOW, 100,
/// ).unwrap();
/// ```
pub fn adjust_window_rect_ex_for_dpi_copy(rect: impl Into<RECT>, style: WS, menu: impl Into<bool>, ex_style: WS_EX, dpi: u32) -> Result<RECT, Error> {
    let mut rect = rect.into();
    adjust_window_rect_ex_for_dpi(&mut rect, style, menu, ex_style, dpi)?;
    Ok(rect)
}
