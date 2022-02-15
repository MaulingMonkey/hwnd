#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unaligned_references)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

#[allow(unused_imports)] use winerr::*; // used in docs

#[macro_use] mod _macros;

pub use winapi::shared::minwindef::ATOM;            // TODO: wrap / typeify
pub use winapi::shared::minwindef::HINSTANCE;       // TODO: wrap / typeify
pub use winapi::shared::minwindef::LPARAM;          // OK?
pub use winapi::shared::minwindef::LRESULT;         // OK?
pub use winapi::shared::minwindef::WPARAM;          // OK?

pub use winapi::shared::windef::HWND;               // TODO: wrap / typeify
pub use winapi::shared::windef::HCURSOR;            // TODO: wrap / typeify
pub use winapi::shared::windef::HICON;              // TODO: wrap / typeify
pub use winapi::shared::windef::HBRUSH;             // TODO: wrap / typeify

pub use winapi::um::winuser::WS_OVERLAPPEDWINDOW;   // TODO: wrap / typeify
pub use winapi::um::winuser::WS_EX_TOOLWINDOW;      // TODO: wrap / typeify

#[allow(dead_code)]
type WM     = u32; // Window Message            // TODO: wrap/type
type WS     = u32; // Window Style              // TODO: wrap/type
#[allow(non_camel_case_types)]
type WS_EX  = u32; // Extended Window Style     // TODO: wrap/type



mods! {
    inl mod functions {
        inl mod adjust_window_rect_;
        inl mod destroy_window_;
        inl mod get_x_window;
        inl mod register_class_;
        inl mod set_foreground_window_;
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
}
