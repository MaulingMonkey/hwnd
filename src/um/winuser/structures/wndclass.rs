use crate::*;
use bytemuck::*;
use std::fmt::{self, Debug, Formatter};
use std::ptr::*;



pub type WndProcNonNull = unsafe extern "system" fn (hwnd: HWnd, msg: WM32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
pub type WndProc        = Option<WndProcNonNull>;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassa)\]
/// WNDCLASSA
///
/// ### See Also
/// *   [register_class_a]
/// *   [About Window Classes](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WndClassA<'a> {
    pub style:      WindowStyle,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HIcon<'static>,
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)\]
/// WNDCLASSW
///
/// ### See Also
/// *   [register_class_w]
/// *   [About Window Classes](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WndClassW<'a> {
    pub style:      WindowStyle,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HIcon<'static>,
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa)\]
/// WNDCLASSEXA
///
/// ### See Also
/// *   [register_class_ex_a]
/// *   [About Window Classes](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WndClassExA<'a> {
    pub size:       u32,
    pub style:      WindowStyle,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HIcon<'static>,
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a>>, // TODO: OrAtom types?
    pub hicon_sm:   HIcon<'static>,
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)\]
/// WNDCLASSEXW
///
/// ### See Also
/// *   [register_class_ex_w]
/// *   [About Window Classes](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-window-classes)
#[derive(Clone, Copy)]
#[repr(C)] pub struct WndClassExW<'a> {
    pub size:       u32,
    pub style:      WindowStyle,
    pub wnd_proc:   WndProc,
    pub cls_extra:  i32,
    pub wnd_extra:  i32,
    pub hinstance:  HInstance<'static>,
    pub hicon:      HIcon<'static>,
    pub hcursor:    HCursor<'static>,
    pub background: HBRUSH,     // TODO: lifetime bound handle?
    pub menu_name:  Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub class_name: Option<abistr::CStrNonNull<'a, u16>>, // TODO: OrAtom types?
    pub hicon_sm:   HIcon<'static>,
}

unsafe impl Zeroable for WndClassA<'_> {} // wnd_proc
unsafe impl Zeroable for WndClassW<'_> {} // wnd_proc
unsafe impl Zeroable for WndClassExA<'_> {} // wnd_proc
unsafe impl Zeroable for WndClassExW<'_> {} // wnd_proc

impl Default for WndClassA<'_>   { fn default() -> Self { Self::zeroed() } }
impl Default for WndClassW<'_>   { fn default() -> Self { Self::zeroed() } }
impl Default for WndClassExA<'_> { fn default() -> Self { Self { size: size_of_32::<Self>(), ..Self::zeroed() } } }
impl Default for WndClassExW<'_> { fn default() -> Self { Self { size: size_of_32::<Self>(), ..Self::zeroed() } } }

impl Debug for WndClassA<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WndClassA")
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

impl Debug for WndClassW<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WndClassW")
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

impl Debug for WndClassExA<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WndClassExA")
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

impl Debug for WndClassExW<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WndClassExW")
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
    WndClassA<'_>   => unsafe { winapi::um::winuser::WNDCLASSA },
    WndClassW<'_>   => unsafe { winapi::um::winuser::WNDCLASSW },
    WndClassExA<'_> => unsafe { winapi::um::winuser::WNDCLASSEXA },
    WndClassExW<'_> => unsafe { winapi::um::winuser::WNDCLASSEXW },
}
