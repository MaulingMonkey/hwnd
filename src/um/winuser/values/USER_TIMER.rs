//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)\]
//! USER_TIMER_\* values for [set](set_timer)\[[_coalescable](set_coalescable_timer)\][_timer](set_timer)

#![allow(non_snake_case)]

#[cfg(doc)] use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)\]
/// USER_TIMER_MAXIMUM
/// &mdash;
/// The maximum timer frequency (24.9 days)
///
/// `elapse_ms` will be clamped to this (e.g. rounded down to at most 24.9 days.)
pub const MAXIMUM : u32 = USER_TIMER_MAXIMUM;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)\]
/// USER_TIMER_MINIMUM
/// &mdash;
/// The minimum timer frequency (10 milliseconds)
///
/// `elapse_ms` will be clamped to this (e.g. rounded up to at least 10 milliseconds.)
pub const MINIMUM : u32 = USER_TIMER_MINIMUM;
