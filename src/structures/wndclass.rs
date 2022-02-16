use crate::*;
use bytemuck::*;
use std::fmt::{self, Debug, Formatter};
use std::ptr::*;



pub type WndProcNonNull = unsafe extern "system" fn (hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
pub type WndProc        = Option<WndProcNonNull>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassa)\]
/// WNDCLASSA
///
/// ### See Also
/// *   [register_class_a]
/// *   [About Window Classes](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WNDCLASSA<'a> {
    pub style:      WS,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HICON,      // TODO: lifetime bound handle?
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)\]
/// WNDCLASSW
///
/// ### See Also
/// *   [register_class_w]
/// *   [About Window Classes](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WNDCLASSW<'a> {
    pub style:      WS,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HICON,      // TODO: lifetime bound handle?
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa)\]
/// WNDCLASSEXA
///
/// ### See Also
/// *   [register_class_ex_a]
/// *   [About Window Classes](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WNDCLASSEXA<'a> {
    pub size:       u32,
    pub style:      WS,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HICON,      // TODO: lifetime bound handle?
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub hicon_sm:   HICON,      // TODO: lifetime bound handle?
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)\]
/// WNDCLASSEXW
///
/// ### See Also
/// *   [register_class_ex_w]
/// *   [About Window Classes](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WNDCLASSEXW<'a> {
    pub size:       u32,
    pub style:      WS,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HICON,      // TODO: lifetime bound handle?
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub hicon_sm:   HICON,      // TODO: lifetime bound handle?
}

unsafe impl Zeroable for WNDCLASSA<'_> {} // wnd_proc
unsafe impl Zeroable for WNDCLASSW<'_> {} // wnd_proc
unsafe impl Zeroable for WNDCLASSEXA<'_> {} // wnd_proc
unsafe impl Zeroable for WNDCLASSEXW<'_> {} // wnd_proc

impl Default for WNDCLASSA<'_>   { fn default() -> Self { Self::zeroed() } }
impl Default for WNDCLASSW<'_>   { fn default() -> Self { Self::zeroed() } }
impl Default for WNDCLASSEXA<'_> { fn default() -> Self { Self { size: size_of_32::<Self>(), ..Self::zeroed() } } }
impl Default for WNDCLASSEXW<'_> { fn default() -> Self { Self { size: size_of_32::<Self>(), ..Self::zeroed() } } }

impl Debug for WNDCLASSA<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WNDCLASSA")
            .field("style",         &self.style         )
            .field("wnd_proc",      &self.wnd_proc.map_or(null::<()>(), |f| unsafe { std::mem::transmute(f) }))
            .field("cls_extra",     &self.cls_extra     )
            .field("wnd_extra",     &self.wnd_extra     )
            .field("hinstance",     &self.hinstance     )
            .field("hicon",         &self.hicon         )
            .field("hcursor",       &self.hcursor       )
            .field("background",    &self.background    )
            .field("menu_name",     &self.menu_name     )
            .field("class_name",    &self.class_name    )
            .finish()
    }
}

impl Debug for WNDCLASSW<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WNDCLASSW")
            .field("style",         &self.style         )
            .field("wnd_proc",      &self.wnd_proc.map_or(null::<()>(), |f| unsafe { std::mem::transmute(f) }))
            .field("cls_extra",     &self.cls_extra     )
            .field("wnd_extra",     &self.wnd_extra     )
            .field("hinstance",     &self.hinstance     )
            .field("hicon",         &self.hicon         )
            .field("hcursor",       &self.hcursor       )
            .field("background",    &self.background    )
            .field("menu_name",     &self.menu_name     )
            .field("class_name",    &self.class_name    )
            .finish()
    }
}

impl Debug for WNDCLASSEXA<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WNDCLASSEXA")
            .field("size",          &self.size          )
            .field("style",         &self.style         )
            .field("wnd_proc",      &self.wnd_proc.map_or(null::<()>(), |f| unsafe { std::mem::transmute(f) }))
            .field("cls_extra",     &self.cls_extra     )
            .field("wnd_extra",     &self.wnd_extra     )
            .field("hinstance",     &self.hinstance     )
            .field("hicon",         &self.hicon         )
            .field("hcursor",       &self.hcursor       )
            .field("background",    &self.background    )
            .field("menu_name",     &self.menu_name     )
            .field("class_name",    &self.class_name    )
            .field("hicon_sm",      &self.hicon_sm      )
            .finish()
    }
}

impl Debug for WNDCLASSEXW<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WNDCLASSEXW")
            .field("size",          &self.size          )
            .field("style",         &self.style         )
            .field("wnd_proc",      &self.wnd_proc.map_or(null::<()>(), |f| unsafe { std::mem::transmute(f) }))
            .field("cls_extra",     &self.cls_extra     )
            .field("wnd_extra",     &self.wnd_extra     )
            .field("hinstance",     &self.hinstance     )
            .field("hicon",         &self.hicon         )
            .field("hcursor",       &self.hcursor       )
            .field("background",    &self.background    )
            .field("menu_name",     &self.menu_name     )
            .field("class_name",    &self.class_name    )
            .field("hicon_sm",      &self.hicon_sm      )
            .finish()
    }
}

convert! {
    WNDCLASSA<'_>   => unsafe { winapi::um::winuser::WNDCLASSA },
    WNDCLASSW<'_>   => unsafe { winapi::um::winuser::WNDCLASSW },
    WNDCLASSEXA<'_> => unsafe { winapi::um::winuser::WNDCLASSEXA },
    WNDCLASSEXW<'_> => unsafe { winapi::um::winuser::WNDCLASSEXW },
}
