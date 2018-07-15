use sys;

pub fn encoder_version() -> i32 {
    (unsafe { sys::WebPGetEncoderVersion() }) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_version() {
        assert_eq!(encoder_version(), 0x10000);
    }
}
