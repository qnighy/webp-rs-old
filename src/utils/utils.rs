use std::os::raw::*;
extern "C" {
    pub(crate) fn WebPSafeMalloc(nmemb: u64, size: usize) -> *mut c_void;
    pub(crate) fn WebPSafeCalloc(nmemb: u64, size: usize) -> *mut c_void;
    pub(crate) fn WebPSafeFree(ptr: *mut c_void);
}
