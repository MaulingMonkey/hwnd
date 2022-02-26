macro_rules! enums { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, i32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("GWL_").unwrap(), $cpp),
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
