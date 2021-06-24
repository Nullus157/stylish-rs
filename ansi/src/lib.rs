#![no_std]
//! [`stylish`] helpers for writing styles as ANSI escape codes.

#[cfg(doc)]
extern crate self as stylish;

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
