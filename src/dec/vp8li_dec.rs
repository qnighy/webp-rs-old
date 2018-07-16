use std::os::raw::*;

pub(crate) type VP8LDecoder = c_void; // TODO

extern "C" {
    pub(crate) fn VP8LDelete(dec: *mut VP8LDecoder);
}
