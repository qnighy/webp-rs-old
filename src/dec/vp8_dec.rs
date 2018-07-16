use std::os::raw::*;

use decode::WEBP_DECODER_ABI_VERSION;

pub(crate) type VP8IoPutHook = Option<extern "C" fn(*const VP8Io) -> c_int>;
pub(crate) type VP8IoSetupHook = Option<extern "C" fn(*mut VP8Io) -> c_int>;
pub(crate) type VP8IoTeardownHook = Option<extern "C" fn(*const VP8Io)>;

#[repr(C)]
pub struct VP8Io {
    // set by VP8GetHeaders()
    pub(crate) width: c_int,
    pub(crate) height: c_int, // picture dimensions, in pixels (invariable).
    // These are the original, uncropped dimensions.
                              // The actual area passed to put() is stored
                              // in mb_w / mb_h fields.

    // set before calling put()
    pub(crate) mb_y: c_int,      // position of the current rows (in pixels)
    pub(crate) mb_w: c_int,      // number of columns in the sample
    pub(crate) mb_h: c_int,      // number of rows in the sample
    pub(crate) y: *const u8,     // rows to copy (in yuv420 format)
    pub(crate) u: *const u8,     // rows to copy (in yuv420 format)
    pub(crate) v: *const u8,     // rows to copy (in yuv420 format)
    pub(crate) y_stride: c_int,  // row stride for luma
    pub(crate) uv_stride: c_int, // row stride for chroma

    pub(crate) opaque: *mut c_void, // user data

    // called when fresh samples are available. Currently, samples are in
    // YUV420 format, and can be up to width x 24 in size (depending on the
    // in-loop filtering level, e.g.). Should return false in case of error
    // or abort request. The actual size of the area to update is mb_w x mb_h
    // in size, taking cropping into account.
    pub(crate) put: VP8IoPutHook,

    // called just before starting to decode the blocks.
    // Must return false in case of setup error, true otherwise. If false is
    // returned, teardown() will NOT be called. But if the setup succeeded
    // and true is returned, then teardown() will always be called afterward.
    pub(crate) setup: VP8IoSetupHook,

    // Called just after block decoding is finished (or when an error occurred
    // during put()). Is NOT called if setup() failed.
    pub(crate) teardown: VP8IoTeardownHook,

    // this is a recommendation for the user-side yuv->rgb converter. This flag
    // is set when calling setup() hook and can be overwritten by it. It then
    // can be taken into consideration during the put() method.
    pub(crate) fancy_upsampling: c_int,

    // Input buffer.
    pub(crate) data_size: usize,
    pub(crate) data: *const u8,

    // If true, in-loop filtering will not be performed even if present in the
    // bitstream. Switching off filtering may speed up decoding at the expense
    // of more visible blocking. Note that output will also be non-compliant
    // with the VP8 specifications.
    pub(crate) bypass_filtering: c_int,

    // Cropping parameters.
    pub(crate) use_cropping: c_int,
    pub(crate) crop_left: c_int,
    pub(crate) crop_right: c_int,
    pub(crate) crop_top: c_int,
    pub(crate) crop_bottom: c_int,

    // Scaling parameters.
    pub(crate) use_scaling: c_int,
    pub(crate) scaled_width: c_int,
    pub(crate) scaled_height: c_int,

    // If non NULL, pointer to the alpha data (if present) corresponding to the
    // start of the current row (That is: it is pre-offset by mb_y and takes
    // cropping into account).
    pub(crate) a: *const u8,
}

extern "C" {
    pub(crate) fn VP8InitIoInternal(io: *mut VP8Io, version: c_int) -> c_int;
}

#[inline]
pub(crate) unsafe fn VP8InitIo(io: *mut VP8Io) -> c_int {
    VP8InitIoInternal(io, WEBP_DECODER_ABI_VERSION as c_int)
}
