use crate::*;
use winapi::shared::minwindef::BOOL;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)\]
/// SetUserObjectInformationW(GetCurrentProcess(), UOI_TIMERPROC_EXCEPTION_SUPPRESSION, ...)
///
/// Controls wheither [`TimerProc`]s are wrapped in an exception handler that suppresses/discards all exceptions
/// (`true` &mdash; default as of Windows 2000, although this might change in the future)
/// or if said exceptions can percolate out of [`get_message_w`] etc. presumably triggering debuggers, crash handlers, etc.
/// (`false` &mdash; **recommended by Microsoft for predictability and security purposes**.)
///
/// ### Errors
/// Unlikely to fail on modern Windows in practice.  This could *theoretically* fail if:
/// *   Configuring TimerProc suppression is disabled by a new policy, security flags, debugger, hooking/patching functions, etc.
/// *   Running on pre-NT Windows, WINE, ReactOS, or other non-Windows system (with stubbed `SetUserObjectInformationW`?)
///
/// ### Examples
/// ```
/// # use hwnd::*;
/// #
/// // As recommended by Microsoft for predictability and security purposes
/// // (call before `set_timer` etc.)
/// let _ = set_timerproc_exception_suppression(false);
/// # set_timerproc_exception_suppression(false).unwrap();
///
/// // Restore default behavior, not recommended:
/// let _ = set_timerproc_exception_suppression(true);
/// # set_timerproc_exception_suppression(true).unwrap();
/// ```
///
/// ### See Also
/// *   [`set_timer`]
/// *   [`set_coalescable_timer`]
/// *   [`TimerProc`]
/// *   [`TimerProcNonNull`]
///
/// ### `SetTimer` Remarks
///
/// Before using `SetTimer` or other timer-related functions, it is recommended to set the `UOI_TIMERPROC_EXCEPTION_SUPPRESSION` flag to `false` through the `SetUserObjectInformationW` function, otherwise the application could behave unpredictably and could be vulnerable to security exploits. For more info, see [SetUserObjectInformationW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw).
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer#remarks>
///
/// ### `SetUserObjectInformationW` Parameters
///
/// If TRUE, Windows will enclose its calls to [TimerProc](https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nc-winuser-timerproc) with an exception handler that consumes and discards all exceptions. This has been the default behavior since Windows 2000, although that may change in future versions of Windows.
///
/// If \[...\] FALSE, Windows will not enclose its calls to [TimerProc](https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nc-winuser-timerproc) with an exception handler. A setting of FALSE is recommended. Otherwise, the application could behave unpredictably, and could be more vulnerable to security exploits.
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw#parameters>
pub fn set_timerproc_exception_suppression(suppress: bool) -> Result<(), Error> {
    fn_context!(set_timerproc_exception_suppression => SetUserObjectInformationW);
    let mut suppress : BOOL = if suppress { 1 } else { 0 };
    let suppress : *mut BOOL = &mut suppress;
    fn_succeeded!(unsafe { SetUserObjectInformationW(GetCurrentProcess(), UOI_TIMERPROC_EXCEPTION_SUPPRESSION as _, suppress.cast(), size_of_32::<BOOL>()) })
}
