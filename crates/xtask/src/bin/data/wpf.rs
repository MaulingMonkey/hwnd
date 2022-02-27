macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("WPF_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    WPF_SETMINPOSITION,
    WPF_RESTORETOMAXIMIZED,
    WPF_ASYNCWINDOWPLACEMENT,
}
