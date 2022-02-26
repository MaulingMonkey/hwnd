macro_rules! enums { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, i32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("GWLP_").unwrap(), $cpp),
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
