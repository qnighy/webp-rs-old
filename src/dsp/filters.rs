use dsp::dsp::{WebPUnfilterFunc, WEBP_FILTER_LAST};

extern "C" {
    pub(crate) static WebPUnfilters: [WebPUnfilterFunc; WEBP_FILTER_LAST as usize];
    pub(crate) fn VP8FiltersInit();
}
