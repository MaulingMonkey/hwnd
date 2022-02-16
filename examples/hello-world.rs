#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_op_in_unsafe_fn)]

use hwnd::*;

use abistr::cstr16;

use bytemuck::*;

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::ptr::*;



fn main() {
    let hinstance = get_module_handle_entry_exe().unwrap();
    let hcursor = load_cursor_w(None, IDC::ARROW).unwrap();

    let wc = WndClassW {
        wnd_proc: Some(window_proc),
        hinstance,
        hcursor,
        class_name: cstr16!("SampleWndClass").into(),
        .. WndClassW::zeroed()
    };
    let wc = unsafe { register_class_w(&wc) }.unwrap();

    let ex_style = 0;
    let style = WS_OVERLAPPEDWINDOW;
    let size = adjust_window_rect_ex_copy(
        Rect { left: 0, right: 800, top: 0, bottom: 600 },
        style, false, ex_style
    ).unwrap();

    let hwnd = unsafe { create_window_ex_w(
        ex_style,
        wc,
        cstr16!("hello-world"),
        style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        size.right - size.left,
        size.bottom - size.top,
        null_mut(),
        null_mut(),
        hinstance,
        null_mut(),
    )}.unwrap();

    show_window_async(hwnd, SW::SHOWNORMAL).unwrap();

    let mut msg = MSG { message: 0, hwnd: null_mut(), time: 0, pt: POINT { x: 0, y: 0 }, lParam: 0, wParam: 0 };
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } != 0 {
        unsafe { TranslateMessage(&msg) };
        unsafe { DispatchMessageW(&msg) };
    }
}

/// ### ⚠️ Safety ⚠️
/// *   `hwnd` must be a valid window
/// *   `wparam` / `lparam` may be assumed to be valid pointers depending no the exact `umsg` passed
unsafe extern "system" fn window_proc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_LBUTTONDOWN => {
            // This blocks, without preventing `hwnd` from being closed, allowing me
            // to experiment with yanking HWNDs out from under recursive wndprocs.
            unsafe { MessageBoxA(null_mut(), "Message Box\0".as_ptr().cast(), "Caption\0".as_ptr().cast(), MB_OK) };
            0
        },
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        },
        _ => unsafe { DefWindowProcW(hwnd, umsg, wparam, lparam) },
    }
}
