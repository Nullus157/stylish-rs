#![no_std]
//! [`stylish`] helpers for writing styles as HTML elements.

#![allow(uncommon_codepoints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(variant_size_differences)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(doc)]
extern crate self as stylish;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod format;
mod html;
#[cfg(all(feature = "alloc", feature = "macros"))]
mod to_string;
mod util;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod 𓀄 {
    pub use stylish_core::format_args;
}

#[cfg(feature = "alloc")]
pub use self::format::format;
pub use self::html::Html;
#[cfg(all(feature = "alloc", feature = "macros"))]
pub use self::to_string::ToHtmlString;
