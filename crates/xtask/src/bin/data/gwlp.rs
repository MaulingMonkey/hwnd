use std::borrow::Cow;

macro_rules! enums { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>, i32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp).into(), stringify!($cpp).replace("GWLP_", "GWLP::").into(), $cpp),
        )*].into_iter()
    }
}}

enums! {
    GWLP_WNDPROC,
    GWLP_HINSTANCE,
    GWLP_HWNDPARENT,
    GWLP_USERDATA,
    GWLP_ID,
}
