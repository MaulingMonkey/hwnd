macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("WS_EX_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    WS_EX_DLGMODALFRAME,
    WS_EX_NOPARENTNOTIFY,
    WS_EX_TOPMOST,
    WS_EX_ACCEPTFILES,
    WS_EX_TRANSPARENT,
    WS_EX_MDICHILD,
    WS_EX_TOOLWINDOW,
    WS_EX_WINDOWEDGE,
    WS_EX_CLIENTEDGE,
    WS_EX_CONTEXTHELP,
    WS_EX_RIGHT,
    WS_EX_LEFT,
    WS_EX_RTLREADING,
    WS_EX_LTRREADING,
    WS_EX_LEFTSCROLLBAR,
    WS_EX_RIGHTSCROLLBAR,
    WS_EX_CONTROLPARENT,
    WS_EX_STATICEDGE,
    WS_EX_APPWINDOW,
    WS_EX_OVERLAPPEDWINDOW,
    WS_EX_PALETTEWINDOW,
    WS_EX_LAYERED,
    WS_EX_NOINHERITLAYOUT,
    WS_EX_NOREDIRECTIONBITMAP,
    WS_EX_LAYOUTRTL,
    WS_EX_COMPOSITED,
    WS_EX_NOACTIVATE,
}
