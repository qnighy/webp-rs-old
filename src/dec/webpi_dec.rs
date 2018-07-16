use std::os::raw::*;

use dec::vp8_dec::VP8Io;

pub(crate) type OutputFunc = Option<extern "C" fn(*const VP8Io, *mut WebPDecParams) -> c_int>;
pub(crate) type OutputAlphaFunc =
    Option<extern "C" fn(*const VP8Io, *mut WebPDecParams, c_int) -> c_int>;
pub(crate) type OutputRowFunc = Option<extern "C" fn(*mut WebPDecParams, c_int, c_int)>;

#[repr(C)]
pub(crate) struct WebPDecParams {
    // pub(crate) output: *mut WebPDecBuffer, // output buffer.
    _output: *mut c_void,      // TODO
    pub(crate) tmp_y: *mut u8, // cache for the fancy upsampler
    pub(crate) tmp_u: *mut u8, // or used for tmp rescaling
    pub(crate) tmp_v: *mut u8,

    pub(crate) last_y: c_int, // coordinate of the line that was last output
    // pub(crate) options: *const WebPDecoderOptions, // if not NULL, use alt decoding features
    _options: *const c_void, // TODO

    // pub(crate) scaler_y: *mut WebPRescaler, // rescalers
    // pub(crate) scaler_u: *mut WebPRescaler,
    // pub(crate) scaler_v: *mut WebPRescaler,
    // pub(crate) scaler_a: *mut WebPRescaler,
    _scaler_y: *mut c_void,         // TODO
    _scaler_u: *mut c_void,         // TODO
    _scaler_v: *mut c_void,         // TODO
    _scaler_a: *mut c_void,         // TODO
    pub(crate) memory: *mut c_void, // overall scratch memory for the output work.

    pub(crate) emit: OutputFunc,              // output RGB or YUV samples
    pub(crate) emit_alpha: OutputAlphaFunc,   // output alpha channel
    pub(crate) emit_alpha_row: OutputRowFunc, // output one line of rescaled alpha values
}
