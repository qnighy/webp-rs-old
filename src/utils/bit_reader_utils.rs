#![allow(dead_code)]

use std::os::raw::*;

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        // x86 32bit
        pub(crate) const BITS: usize = 24;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u32;
    } else if #[cfg(target_arch = "x86_64")] {
        // x86 64bit
        pub(crate) const BITS: usize = 56;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u64;
    } else if #[cfg(target_arch = "arm")] {
        // ARM
        pub(crate) const BITS: usize = 24;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u32;
    } else if #[cfg(target_arch = "aarch64")] {
        // ARM 64bit
        pub(crate) const BITS: usize = 56;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u64;
    } else if #[cfg(target_arch = "mips")] {
        // MIPS
        pub(crate) const BITS: usize = 24;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u32;
    } else {
        // reasonable default
        pub(crate) const BITS: usize = 24;
        #[allow(non_camel_case_types)]
        pub(crate) type bit_t = u32;
    }
}

#[allow(non_camel_case_types)]
pub(crate) type range_t = u32;

#[repr(C)]
pub(crate) struct VP8BitReader {
    // boolean decoder  (keep the field ordering as is!)
    pub(crate) value_: bit_t,   // current value
    pub(crate) range_: range_t, // current range minus 1. In [127, 254] interval.
    pub(crate) bits_: c_int,    // number of valid bits left
    // read buffer
    pub(crate) buf_: *const u8,     // next byte to be read
    pub(crate) buf_end_: *const u8, // end of read buffer
    pub(crate) buf_max_: *const u8, // max packed-read position on buffer
    pub(crate) eof_: c_int,         // true if input is exhausted
}

#[allow(non_snake_case)]
#[inline]
pub(crate) unsafe extern "C" fn VP8Get(br: *mut VP8BitReader) -> u32 {
    VP8GetValue(br, 1)
}

extern "C" {
    pub(crate) fn VP8GetValue(br: *mut VP8BitReader, num_bits: c_int) -> u32;
    pub(crate) fn VP8GetSignedValue(br: *mut VP8BitReader, num_bits: c_int) -> i32;
}
