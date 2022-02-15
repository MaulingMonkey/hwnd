pub(crate) const fn size_of_32<T>() -> u32 {
    let size = std::mem::size_of::<T>();
    assert!(size <= std::u32::MAX as _);
    size as _
}
