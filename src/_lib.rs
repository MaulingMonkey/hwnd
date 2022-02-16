#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unaligned_references)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

#[allow(unused_imports)] use winerr::*; // used in docs

#[macro_use] mod _macros;

pub use winapi::shared::minwindef::LPARAM;          // OK?
pub use winapi::shared::minwindef::LRESULT;         // OK?
pub use winapi::shared::minwindef::WPARAM;          // OK?

pub use winapi::shared::windef::HWND;               // TODO: wrap / typeify
pub use winapi::shared::windef::HCURSOR;            // TODO: wrap / typeify
pub use winapi::shared::windef::HMENU;              // TODO: wrap / typeify
pub use winapi::shared::windef::HICON;              // TODO: wrap / typeify
pub use winapi::shared::windef::HBRUSH;             // TODO: wrap / typeify

pub use winapi::um::winuser::WS_OVERLAPPEDWINDOW;   // TODO: wrap / typeify
pub use winapi::um::winuser::WS_EX_TOOLWINDOW;      // TODO: wrap / typeify

pub use winapi::um::winuser::GWL_STYLE;             // TODO: wrap / typeify / replace
pub use winapi::um::winuser::GWLP_WNDPROC;          // TODO: wrap / typeify / replace

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)\]
/// HWND_MESSAGE
///
/// Can be passed as the `parent` to [create_window_a] etc. to create a message-only window, which:
/// *   Is not visible
/// *   Has no Z-order
/// *   Cannot be enumerated
/// *   Does not receive broadcast messages
pub use winapi::um::winuser::HWND_MESSAGE;          // TODO: wrap / typeify

#[allow(dead_code)]
type WM     = u32; // Window Message            // TODO: wrap/type
type WS     = u32; // Window Style              // TODO: wrap/type
#[allow(non_camel_case_types)]
type WS_EX  = u32; // Extended Window Style     // TODO: wrap/type



mods! {
    inl mod functions {
        inl mod adjust_window_rect_;
        inl mod close_window_;
        inl mod create_window_;
        inl mod destroy_window_;
        inl mod get_client_rect_;
        inl mod get_current_x_id;
        inl mod get_module_handle_;
        inl mod get_window_thread_process_id_;
        inl mod get_window;
        inl mod get_x_window;
        inl mod is;
        inl mod register_class_;
        inl mod set_foreground_window_;
        inl mod show_window_;
    }

    inl mod structures {
        inl mod error;
        inl mod name_or_atom;
        inl mod rect;
        inl mod wndclass;
    }

    inl mod utils {
        inl mod _32;
    }

    inl mod values {
        inl mod atom;
    }
}
