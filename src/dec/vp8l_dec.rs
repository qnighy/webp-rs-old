use std::os::raw::*;

use dec::alpha_dec::ALPHDecoder;

extern "C" {
    pub(crate) fn VP8LDecodeAlphaHeader(
        alph_dec: *mut ALPHDecoder,
        data: *const u8,
        data_size: usize,
    ) -> c_int;

    pub(crate) fn VP8LDecodeAlphaImageStream(alph_dec: *mut ALPHDecoder, last_row: c_int) -> c_int;
}
