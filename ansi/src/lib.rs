#![no_std]
//! [`stylish`] helpers for writing styles as ANSI escape codes.

#![allow(uncommon_codepoints)]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(doc)]
extern crate self as stylish;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod ansi;
#[cfg(feature = "alloc")]
mod format;
#[cfg(feature = "std")]
pub mod io;
#[cfg(all(feature = "alloc", feature = "macros"))]
mod to_string;
mod util;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod 𓀄 {
    pub use stylish_core::format_args;
}

pub use self::ansi::Ansi;
#[cfg(feature = "alloc")]
pub use self::format::format;
#[cfg(all(feature = "alloc", feature = "macros"))]
pub use self::to_string::ToAnsiString;
