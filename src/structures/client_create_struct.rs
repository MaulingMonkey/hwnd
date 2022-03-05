use crate::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-clientcreatestruct)\]
/// CLIENTCREATESTRUCT
///
/// Contains information about the menu and first multiple-document interface (MDI) child window of an MDI client window.
/// An application passes a pointer to this structure as the lpParam parameter of the CreateWindow function when creating an MDI client window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroable)]
#[repr(C)] pub struct ClientCreateStruct {
    pub window_menu:    HMENU, // header uses HANDLE but HMENU seems more appropriate based on docs etc.
    pub id_first_child: u32,
}

impl Default for ClientCreateStruct {
    fn default() -> Self { Self::zeroed() }
}

// no winapi equivalent
// convert!(ClientCreateStruct <=> unsafe { winapi::um::winuser::CLIENTCREATESTRUCT });
