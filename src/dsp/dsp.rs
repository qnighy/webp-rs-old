use std::os::raw::*;

pub(crate) use self::WEBP_FILTER_TYPE::*;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) enum WEBP_FILTER_TYPE {
    // Filter types.
    WEBP_FILTER_NONE = 0,
    WEBP_FILTER_HORIZONTAL,
    WEBP_FILTER_VERTICAL,
    WEBP_FILTER_GRADIENT,
    WEBP_FILTER_LAST = WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT as isize + 1, // end marker
    WEBP_FILTER_BEST,                                                       // meta-types
    WEBP_FILTER_FAST,
}

impl Default for WEBP_FILTER_TYPE {
    fn default() -> Self {
        WEBP_FILTER_NONE
    }
}

impl WEBP_FILTER_TYPE {
    pub(crate) fn new(x: i32) -> Self {
        macro_rules! gen {
            ($e:expr) => {
                if x == $e as i32 {
                    return $e;
                }
            };
        }
        gen!(WEBP_FILTER_NONE);
        gen!(WEBP_FILTER_HORIZONTAL);
        gen!(WEBP_FILTER_VERTICAL);
        gen!(WEBP_FILTER_GRADIENT);
        c_assert!(false);
        WEBP_FILTER_NONE
    }
}

pub(crate) type WebPFilterFunc = Option<extern "C" fn(*const u8, c_int, c_int, c_int, *mut u8)>;

// In-place un-filtering.
// Warning! 'prev_line' pointer can be equal to 'cur_line' or 'preds'.
pub(crate) type WebPUnfilterFunc = Option<extern "C" fn(*const u8, *const u8, *mut u8, c_int)>;
