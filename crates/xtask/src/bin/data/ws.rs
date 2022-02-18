macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("WS_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    WS_OVERLAPPED,
    WS_POPUP,
    WS_CHILD,
    WS_MINIMIZE,
    WS_VISIBLE,
    WS_DISABLED,
    WS_CLIPSIBLINGS,
    WS_CLIPCHILDREN,
    WS_MAXIMIZE,
    WS_CAPTION,
    WS_BORDER,
    WS_DLGFRAME,
    WS_VSCROLL,
    WS_HSCROLL,
    WS_SYSMENU,
    WS_THICKFRAME,
    WS_GROUP,
    WS_TABSTOP,
    WS_MINIMIZEBOX,
    WS_MAXIMIZEBOX,
    WS_TILED,
    WS_ICONIC,
    WS_SIZEBOX,
    WS_TILEDWINDOW,
    WS_OVERLAPPEDWINDOW,
    WS_POPUPWINDOW,
    WS_CHILDWINDOW,
}
