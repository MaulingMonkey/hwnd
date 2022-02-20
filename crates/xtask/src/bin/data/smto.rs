macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("SMTO_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    SMTO_ABORTIFHUNG,
    SMTO_BLOCK,
    SMTO_NORMAL,
    SMTO_NOTIMEOUTIFNOTHUNG,
    SMTO_ERRORONEXIT,
}
