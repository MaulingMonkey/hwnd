use crate::*;
use bytemuck::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)\]
/// MSG
///
/// Contains message information from a thread's message queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)]
#[repr(C)] pub struct Msg {
    pub hwnd:       HWnd,
    pub message:    WM32,
    // NOTE: 32 bits of padding on 64-bit
    pub wparam:     WPARAM,
    pub lparam:     LPARAM,
    pub time:       u32,
    pub pt:         Point,
    #[cfg(any(target_os = "macos"))] pub private: u32,
    // NOTE: 32 bits of padding on !macos 64-bit
}

convert!(Msg <=> unsafe { winapi::um::winuser::MSG });

impl Msg {
    pub fn default() -> Self { Default::default() }
    pub fn zeroed()  -> Self { Zeroable::zeroed() }
}
