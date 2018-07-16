use std::os::raw::*;

use sys;
use WebpBox;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum VP8StatusCode {
    VP8_STATUS_OK = 0,
    VP8_STATUS_OUT_OF_MEMORY,
    VP8_STATUS_INVALID_PARAM,
    VP8_STATUS_BITSTREAM_ERROR,
    VP8_STATUS_UNSUPPORTED_FEATURE,
    VP8_STATUS_SUSPENDED,
    VP8_STATUS_USER_ABORT,
    VP8_STATUS_NOT_ENOUGH_DATA,
}

pub fn decoder_version() -> i32 {
    (unsafe { sys::WebPGetDecoderVersion() }) as i32
}

pub fn get_info(data: &[u8]) -> Option<(u32, u32)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };
    if result != 0 {
        Some((width as u32, height as u32))
    } else {
        None
    }
}

macro_rules! wrap_decoder {
    ($decoder:ident, $c_decoder:ident, $elems:expr) => {
        pub fn $decoder(data: &[u8]) -> Option<(u32, u32, WebpBox<[u8]>)> {
            let mut width: c_int = 0;
            let mut height: c_int = 0;
            let result =
                unsafe { sys::$c_decoder(data.as_ptr(), data.len(), &mut width, &mut height) };
            if !result.is_null() {
                let width = width as u32;
                let height = height as u32;
                let len = width as usize * height as usize * $elems;
                let ptr = unsafe { WebpBox::from_raw_parts(result, len) };
                Some((width, height, ptr))
            } else {
                None
            }
        }
    };
}

wrap_decoder!(decode_rgba, WebPDecodeRGBA, 4);
wrap_decoder!(decode_argb, WebPDecodeARGB, 4);
wrap_decoder!(decode_bgra, WebPDecodeBGRA, 4);
wrap_decoder!(decode_rgb, WebPDecodeRGB, 3);
wrap_decoder!(decode_bgr, WebPDecodeBGR, 3);

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter;

    fn example_webp() -> Vec<u8> {
        include_bytes!("../libwebp-1.0.0/examples/test.webp").to_vec()
    }

    fn reference_ppm() -> Vec<u8> {
        include_bytes!("../libwebp-1.0.0/examples/test_ref.ppm").to_vec()
    }

    fn reference_rgb() -> Vec<u8> {
        let mut vec = reference_ppm();

        let header = b"P6\n128 128\n255\n";
        assert_eq!(&vec[..header.len()], header);

        vec.drain(..header.len());
        vec
    }

    fn reference_rgba() -> Vec<u8> {
        reference_rgb()
            .chunks(3)
            .flat_map(|rgb| {
                iter::once(rgb[0])
                    .chain(iter::once(rgb[1]))
                    .chain(iter::once(rgb[2]))
                    .chain(iter::once(255))
            })
            .collect()
    }

    fn reference_argb() -> Vec<u8> {
        reference_rgb()
            .chunks(3)
            .flat_map(|rgb| {
                iter::once(255)
                    .chain(iter::once(rgb[0]))
                    .chain(iter::once(rgb[1]))
                    .chain(iter::once(rgb[2]))
            })
            .collect()
    }

    fn reference_bgra() -> Vec<u8> {
        reference_rgb()
            .chunks(3)
            .flat_map(|rgb| {
                iter::once(rgb[2])
                    .chain(iter::once(rgb[1]))
                    .chain(iter::once(rgb[0]))
                    .chain(iter::once(255))
            })
            .collect()
    }

    fn reference_bgr() -> Vec<u8> {
        reference_rgb()
            .chunks(3)
            .flat_map(|rgb| {
                iter::once(rgb[2])
                    .chain(iter::once(rgb[1]))
                    .chain(iter::once(rgb[0]))
            })
            .collect()
    }

    #[test]
    fn test_decoder_version() {
        assert_eq!(decoder_version(), 0x10000);
    }

    #[test]
    fn test_get_info() {
        assert_eq!(get_info(&example_webp()), Some((128, 128)));
        assert_eq!(get_info(b""), None);
    }

    #[test]
    fn test_decode_rgba() {
        let (width, height, buf) = decode_rgba(&example_webp()).unwrap();
        assert_eq!((width, height), (128, 128));
        let reference_image = reference_rgba();
        assert_abs_diff_eq!(*buf.as_ref(), *reference_image.as_slice(), epsilon = 1);
    }

    #[test]
    fn test_decode_argb() {
        let (width, height, buf) = decode_argb(&example_webp()).unwrap();
        assert_eq!((width, height), (128, 128));
        let reference_image = reference_argb();
        assert_abs_diff_eq!(*buf.as_ref(), *reference_image.as_slice(), epsilon = 1);
    }

    #[test]
    fn test_decode_bgra() {
        let (width, height, buf) = decode_bgra(&example_webp()).unwrap();
        assert_eq!((width, height), (128, 128));
        let reference_image = reference_bgra();
        assert_abs_diff_eq!(*buf.as_ref(), *reference_image.as_slice(), epsilon = 1);
    }

    #[test]
    fn test_decode_rgb() {
        let (width, height, buf) = decode_rgb(&example_webp()).unwrap();
        assert_eq!((width, height), (128, 128));
        let reference_image = reference_rgb();
        assert_abs_diff_eq!(*buf.as_ref(), *reference_image.as_slice(), epsilon = 1);
    }

    #[test]
    fn test_decode_bgr() {
        let (width, height, buf) = decode_bgr(&example_webp()).unwrap();
        assert_eq!((width, height), (128, 128));
        let reference_image = reference_bgr();
        assert_abs_diff_eq!(*buf.as_ref(), *reference_image.as_slice(), epsilon = 1);
    }

    #[test]
    fn test_decode_rgba2() {
        let img = include_bytes!("../examples/rust-logo-256x256.webp");
        let (width, height, buf) = decode_rgba(img).unwrap();
        assert_eq!((width, height), (256, 256));
        assert_abs_diff_eq!(
            buf.as_ref()[10000..10100],
            [
                151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0,
                151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0,
                151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0,
                151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0,
                151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0, 151, 96, 58, 0
            ][..],
            epsilon = 1
        );
        assert_abs_diff_eq!(
            buf.as_ref()[100000..100100],
            [
                93, 73, 57, 255, 93, 73, 57, 255, 93, 73, 57, 255, 93, 73, 57, 255, 94, 72, 57,
                255, 94, 72, 57, 255, 94, 72, 57, 255, 94, 72, 57, 255, 94, 72, 57, 255, 94, 72,
                57, 255, 94, 72, 57, 255, 94, 72, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255, 96,
                71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255,
                96, 71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 255, 96, 71, 57, 16,
                98, 71, 55, 0
            ][..],
            epsilon = 1
        );
    }
}
