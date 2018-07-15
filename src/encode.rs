use std::os::raw::*;
use std::ptr;

use sys;
use WebpBox;

pub fn encoder_version() -> i32 {
    (unsafe { sys::WebPGetEncoderVersion() }) as i32
}

macro_rules! wrap_encoder {
    ($encoder:ident, $c_encoder:ident, $elems:expr) => {
        pub fn $encoder(
            buf: &[u8],
            width: u32,
            height: u32,
            stride: u32,
            quality_factor: f32,
        ) -> Option<WebpBox<[u8]>> {
            let rgb_len = (stride as usize).checked_mul(height as usize).unwrap();
            assert_eq!(rgb_len, buf.len());
            assert!(width <= stride / $elems);
            assert_eq!(width as c_int as u32, width);
            assert_eq!(height as c_int as u32, height);
            assert_eq!(stride as c_int as u32, stride);

            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                sys::$c_encoder(
                    buf.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    quality_factor as c_float,
                    &mut output,
                )
            };
            if result != 0 {
                let len = result;
                Some(unsafe { WebpBox::from_raw_parts(output, len) })
            } else {
                None
            }
        }
    };
}

wrap_encoder!(encode_rgb, WebPEncodeRGB, 3);
wrap_encoder!(encode_bgr, WebPEncodeBGR, 3);
wrap_encoder!(encode_rgba, WebPEncodeRGBA, 4);
wrap_encoder!(encode_bgra, WebPEncodeBGRA, 4);

macro_rules! wrap_lossless_encoder {
    ($encoder:ident, $c_encoder:ident, $elems:expr) => {
        pub fn $encoder(buf: &[u8], width: u32, height: u32, stride: u32) -> Option<WebpBox<[u8]>> {
            let rgb_len = (stride as usize).checked_mul(height as usize).unwrap();
            assert_eq!(rgb_len, buf.len());
            assert!(width <= stride / $elems);
            assert_eq!(width as c_int as u32, width);
            assert_eq!(height as c_int as u32, height);
            assert_eq!(stride as c_int as u32, stride);

            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                sys::$c_encoder(
                    buf.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    &mut output,
                )
            };
            if result != 0 {
                let len = result;
                Some(unsafe { WebpBox::from_raw_parts(output, len) })
            } else {
                None
            }
        }
    };
}

wrap_lossless_encoder!(encode_lossless_rgb, WebPEncodeLosslessRGB, 3);
wrap_lossless_encoder!(encode_lossless_bgr, WebPEncodeLosslessBGR, 3);
wrap_lossless_encoder!(encode_lossless_rgba, WebPEncodeLosslessRGBA, 4);
wrap_lossless_encoder!(encode_lossless_bgra, WebPEncodeLosslessBGRA, 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_version() {
        assert_eq!(encoder_version(), 0x10000);
    }

    #[test]
    fn test_encode_rgb() {
        let img = encode_rgb(&[255, 255, 255], 1, 1, 3, 75.0);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_bgr() {
        let img = encode_bgr(&[255, 255, 255], 1, 1, 3, 75.0);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_rgba() {
        let img = encode_rgba(&[255, 255, 255, 255], 1, 1, 4, 75.0);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_bgra() {
        let img = encode_bgra(&[255, 255, 255, 255], 1, 1, 4, 75.0);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_lossless_rgb() {
        let img = encode_lossless_rgb(&[255, 255, 255], 1, 1, 3);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_lossless_bgr() {
        let img = encode_lossless_bgr(&[255, 255, 255], 1, 1, 3);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_lossless_rgba() {
        let img = encode_lossless_rgba(&[255, 255, 255, 255], 1, 1, 4);
        assert!(img.is_some());
    }

    #[test]
    fn test_encode_lossless_bgra() {
        let img = encode_lossless_bgra(&[255, 255, 255, 255], 1, 1, 4);
        assert!(img.is_some());
    }
}
