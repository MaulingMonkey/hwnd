use crate::*;
use crate::SW::ShowWindowCmd;
use crate::WPF::WindowPlacementFlags;
use bytemuck::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)\]
/// WINDOWPLACEMENT
///
/// Contains information about the placement of a window on the screen.
///
/// ### See Also
/// *   [get_window_placement]
/// *   [set_window_placement]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)]
#[repr(C)] pub struct WindowPlacement {
    pub length:             u32,
    pub flags:              WindowPlacementFlags,
    pub show_cmd:           ShowWindowCmd,
    pub min_position:       Point,
    pub max_position:       Point,
    pub normal_position:    Rect,
    #[cfg(target_os = "macos")]
    pub device:             Rect,
}

convert!(WindowPlacement <=> unsafe { winapi::um::winuser::WINDOWPLACEMENT });
