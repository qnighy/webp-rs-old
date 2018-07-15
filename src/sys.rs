use std::os::raw::*;

extern "C" {
    pub fn WebPFree(ptr: *mut c_void);

    pub fn WebPGetDecoderVersion() -> c_int;
    pub fn WebPGetInfo(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;

    pub fn WebPDecodeRGBA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;

    pub fn WebPDecodeARGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;

    pub fn WebPDecodeBGRA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;

    pub fn WebPDecodeRGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;

    pub fn WebPDecodeBGR(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;

    pub fn WebPGetEncoderVersion() -> c_int;

    pub fn WebPEncodeRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeLosslessRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeLosslessBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeLosslessRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;

    pub fn WebPEncodeLosslessBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
}
