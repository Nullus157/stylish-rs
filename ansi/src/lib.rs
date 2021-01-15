#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod ansi;
#[cfg(feature = "alloc")]
mod format;
#[cfg(feature = "alloc")]
mod to_string;
mod util;

#[doc(hidden)]
pub mod __export {
    pub use stylish_core::format_args;
}

pub use self::ansi::Ansi;
#[cfg(feature = "alloc")]
pub use self::{format::format, to_string::ToAnsiString};
