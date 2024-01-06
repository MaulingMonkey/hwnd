use std::borrow::Cow;

macro_rules! enums { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>, i32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp).into(), stringify!($cpp).replace("SW_", "SW::").into(), $cpp),
        )*].into_iter()
    }
}}

enums! {
    SW_HIDE,
    SW_SHOWNORMAL,
    SW_NORMAL,
    SW_SHOWMINIMIZED,
    SW_SHOWMAXIMIZED,
    SW_MAXIMIZE,
    SW_SHOWNOACTIVATE,
    SW_SHOW,
    SW_MINIMIZE,
    SW_SHOWMINNOACTIVE,
    SW_SHOWNA,
    SW_RESTORE,
    SW_SHOWDEFAULT,
}
