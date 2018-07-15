use std::os::raw::*;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum WebPWorkerStatus {
    NOT_OK = 0, // object is unusable
    OK,         // ready to work
    WORK,       // busy finishing the current task
}

pub(crate) type WebPWorkerHook = Option<extern "C" fn(*mut c_void, *mut c_void) -> c_int>;

#[repr(C)]
pub(crate) struct WebPWorker {
    pub(crate) impl_: *mut c_void, // platform-dependent implementation worker details
    pub(crate) status_: WebPWorkerStatus,
    pub(crate) hook: WebPWorkerHook, // hook to call
    pub(crate) data1: *mut c_void,   // first argument passed to 'hook'
    pub(crate) data2: *mut c_void,   // second argument passed to 'hook'
    pub(crate) had_error: c_int,     // return value of the last call to 'hook'
}
