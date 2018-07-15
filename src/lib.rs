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
mod encode;
pub mod sys;
#[doc(hidden)]
pub mod utils;
mod webpbox;

pub use decode::*;
pub use encode::*;
pub use webpbox::WebpBox;
