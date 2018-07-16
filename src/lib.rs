#[macro_use]
extern crate cfg_if;

#[cfg(test)]
#[macro_use]
extern crate approx;

macro_rules! c_assert {
    ($cond:expr) => {
        if cfg!(debug_assertion) {
            if !$cond {
                ::std::process::abort()
            }
        }
    };
}

#[doc(hidden)]
pub mod dec;
mod decode;
#[doc(hidden)]
pub mod dsp;
mod encode;
pub mod format_constants;
pub mod sys;
#[doc(hidden)]
pub mod utils;
mod webpbox;

pub use decode::*;
pub use encode::*;
pub use webpbox::WebpBox;
