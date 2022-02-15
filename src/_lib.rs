#[macro_use] mod _macros;

pub use winapi::shared::windef::HWND;
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
        inl mod get_x_window;
    }

    inl mod structures {
        inl mod error;
        inl mod rect;
    }
}
