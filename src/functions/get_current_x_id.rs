use winapi::um::processthreadsapi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-gb/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)\]
/// GetCurrentProcessId
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let process_id = get_current_process_id();
/// assert!(process_id != 0);
/// ```
pub fn get_current_process_id() -> u32 {
    fn_context!(get_current_process_id => GetCurrentThreadId);
    unsafe { GetCurrentProcessId() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-gb/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)\]
/// GetCurrentThreadId
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let thread_id = get_current_thread_id();
/// assert!(thread_id != 0);
/// ```
pub fn get_current_thread_id() -> u32 {
    // XXX: We could make this return NonZeroU32:
    // > Note that no thread identifier will ever be 0.
    // > https://docs.microsoft.com/en-us/windows/win32/procthread/thread-handles-and-identifiers
    // However, that might be awkward to work with.
    // We could have an alternative function...?
    // Also, what if we run on a hacked up emulator that violates this invariant?
    fn_context!(get_current_thread_id => GetCurrentThreadId);
    unsafe { GetCurrentThreadId() }
}
