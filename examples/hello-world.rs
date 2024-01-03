#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_op_in_unsafe_fn)]

use hwnd::*;

use abistr::cstr16;

use bytemuck::*;

use winapi::um::winuser::*;

use winresult::ERROR;

use std::ptr::*;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};



fn main() {
    // To ensure any Drop s are run before calling exit, dispatch
    // all logic to an inner implementation function:
    std::process::exit(main_imp())
}

fn main_imp() -> i32 {
    let hinstance = get_module_handle_entry_exe().unwrap();
    let hcursor = load_cursor_w(None, IDC::ARROW).unwrap();
    let hicon   = load_icon_w(None, IDI::APPLICATION).unwrap();

    let wc = WndClassW {
        wnd_proc: Some(window_proc),
        hinstance,
        hcursor,
        hicon,
        class_name: cstr16!("SampleWndClass").into(),
        .. WndClassW::zeroed()
    };
    let wc = unsafe { register_class_w(&wc) }.unwrap();

    let ex_style = 0;
    let style = WS::OVERLAPPEDWINDOW;
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

    let mut msg = Msg::zeroed();
    while get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap() {
        translate_message(&msg);
        let _ = unsafe { dispatch_message_w(&msg) };
    }

    if msg.message == WM::QUIT { return msg.wparam as _ }
    0 // assume success of we ever `break` out of our message loop instead of `return !0`ing
}

/// ### ⚠️ Safety ⚠️
/// *   `hwnd` must be a valid window
/// *   `wparam` / `lparam` may be assumed to be valid pointers depending no the exact `umsg` passed
unsafe extern "system" fn window_proc(hwnd: HWnd, umsg: WM32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if umsg == WM::GETMINMAXINFO {
        EARLY.set(hwnd, "early").unwrap();
        LATE .set(hwnd, "late" ).unwrap();
    }

    eprintln!("window_proc({hwnd:?}, {umsg:?}");
    eprintln!("  early: {:?}", EARLY.get_copy(hwnd));
    eprintln!("  late:  {:?}", LATE .get_copy(hwnd));

    match umsg {
        WM::LBUTTONDOWN => {
            // This blocks, without preventing `hwnd` from being closed, allowing me
            // to experiment with yanking HWNDs out from under recursive wndprocs.
            unsafe { MessageBoxA(null_mut(), "Message Box\0".as_ptr().cast(), "Caption\0".as_ptr().cast(), MB_OK) };
            0
        },
        WM::RBUTTONDOWN => {
            unsafe { destroy_window(hwnd) }.unwrap();
            assert_eq!(ERROR::INVALID_WINDOW_HANDLE, unsafe { destroy_window(hwnd) }.unwrap_err());
            0
        },
        WM::DESTROY => {
            assert!(is_window(hwnd));
            assert_eq!(ERROR::INVALID_WINDOW_HANDLE, EARLY.set(hwnd, "early").unwrap_err());    // unable to set "early" data: window is being destroyed
            LATE.set(hwnd, "late").unwrap();                                                    // still able to set "late" data

            static DESTROY : AtomicBool = AtomicBool::new(false);
            if !DESTROY.load(Relaxed) {
                DESTROY.store(true, Relaxed);
                unsafe { destroy_window(hwnd) }.unwrap();
                assert_eq!(ERROR::INVALID_WINDOW_HANDLE, LATE.set(hwnd, "late").unwrap_err());  // no longer able to set even "late" data: destroy_window returned
            }
            post_quit_message(0);
            0
        },
        _ => unsafe { def_window_proc_w(hwnd, umsg, wparam, lparam) },
    }
}

static EARLY : assoc::local::Slot<&'static str> = assoc::local::Slot::new_drop_early();
static LATE  : assoc::local::Slot<&'static str> = assoc::local::Slot::new_drop_late();
