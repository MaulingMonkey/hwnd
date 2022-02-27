use std::borrow::Cow;

macro_rules! enums { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>, i32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp).into(), stringify!($cpp).replace("GWL_", "GWL::").into(), $cpp),
        )*].into_iter()
    }
}}

enums! {
    GWL_WNDPROC,
    GWL_HINSTANCE,
    GWL_HWNDPARENT,
    GWL_STYLE,
    GWL_EXSTYLE,
    GWL_USERDATA,
    GWL_ID,
}
