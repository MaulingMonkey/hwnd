//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcoalescabletimer)\]
//! TIMERV_\* values for [set_coalescable_timer]

#![allow(non_snake_case)]

#[cfg(doc)] use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcoalescabletimer)\]
/// TIMERV_DEFAULT_COALESCING
/// &mdash;
/// Uses default timer coalescing.
///
/// ### To be verified
/// As I understand it, this means that multiple timer callbacks will be collapsed into one call if the message loop can't dispatch them fast enough?
pub const DEFAULT_COALESCING : u32 = TIMERV_DEFAULT_COALESCING;

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcoalescabletimer)\]
/// TIMERV_NO_COALESCING
/// &mdash;
/// Disable timer coalescing.
///
/// **Note:** Do not use this value unless you are *certain* that the timer *must not* coalesce.
///
/// ### To be verified
/// As I understand it, this means that if the message loop can't dispatch timer callbacks fast enough,
/// the message queue backlog will grow until e.g. it eventually hits it's limit
/// ([10,000 messages by default](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew#remarks)),
/// resulting in [`ERROR::NOT_ENOUGH_QUOTA`] when attempting to enqueue more messages.
pub const NO_COALESCING : u32 = TIMERV_NO_COALESCING;
