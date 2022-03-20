pub use crate::platform::{DllCache, LPCSTR, LPCWSTR};
use crate::platform::{ULONG_PTR, WORD};
pub use core::{self, option::Option, result::Result};

// Copied MAKEINTRESOURCEA function from winapi so that it can be const
#[inline]
pub const fn make_int_resource_a(i: WORD) -> LPCSTR {
    i as ULONG_PTR as _
}
