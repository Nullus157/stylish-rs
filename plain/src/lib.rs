#![no_std]
//! [`stylish`] helpers for discarding styles.

#![allow(uncommon_codepoints)]

#[cfg(doc)]
extern crate self as stylish;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod format;
mod plain;
#[cfg(feature = "alloc")]
mod to_string;

pub use self::plain::Plain;
#[cfg(feature = "alloc")]
pub use self::{format::format, to_string::ToPlainString};

#[doc(hidden)]
pub mod 𓀄 {
    pub use stylish_core::format_args;
}
