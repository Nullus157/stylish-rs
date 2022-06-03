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
#[cfg(all(feature = "alloc", feature = "macros"))]
mod to_string;

pub use self::plain::Plain;
#[cfg(feature = "alloc")]
pub use self::format::format;
#[cfg(all(feature = "alloc", feature = "macros"))]
pub use self::to_string::ToPlainString;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod 𓀄 {
    pub use stylish_core::format_args;
}
