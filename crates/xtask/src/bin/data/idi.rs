macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, usize)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("IDI_").unwrap(), $cpp as usize),
        )*].into_iter()
    }
}}

flags! {
    IDI_APPLICATION,
    IDI_HAND,
    IDI_QUESTION,
    IDI_EXCLAMATION,
    IDI_ASTERISK,
    IDI_WINLOGO,
    IDI_SHIELD,

    // duplicates
    IDI_WARNING,
    IDI_ERROR,
    IDI_INFORMATION,
}
