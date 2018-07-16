use dec::vp8_dec::VP8Io;
use dec::webpi_dec::WebPDecParams;

extern "C" {
    pub(crate) fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io);
}
