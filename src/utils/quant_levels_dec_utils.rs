use std::os::raw::*;

extern "C" {
    pub(crate) fn WebPDequantizeLevels(
        data: *mut u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        strength: c_int,
    ) -> c_int;
}
