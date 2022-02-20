macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("ISMEX_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    ISMEX_NOSEND,
    ISMEX_CALLBACK,
    ISMEX_NOTIFY,
    ISMEX_REPLIED,
    ISMEX_SEND,
}
