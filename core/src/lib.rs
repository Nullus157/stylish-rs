#![no_std]

//! The semver-stable subset of [`stylish`].
//!
//! See the main documentation in [`stylish`], there is no reason to depend on
//! this crate directly.

#![allow(uncommon_codepoints)]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(doc)]
extern crate self as stylish;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub mod io;

#[macro_use]
mod std_compat;

mod arguments;
mod display;
#[cfg(feature = "alloc")]
mod format;
mod formatter;
#[cfg(feature = "alloc")]
mod string;
#[cfg(all(feature = "alloc", feature = "macros"))]
mod to_string;
mod write;

pub use core::fmt::{Error, Result};

pub use stylish_style::{Background, Color, Foreground, Intensity, Restyle, Style, StyleDiff};

#[cfg(all(feature = "alloc", feature = "macros"))]
pub use self::to_string::ToStylishString;
pub use self::{arguments::Arguments, display::Display, formatter::Formatter, write::Write};
#[cfg(feature = "alloc")]
pub use self::{format::format, string::String};

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod 𓀄 {
    pub use core::{fmt, option::Option};

    pub use stylish_macros::{format_args, format_args_nl};
    pub use with_builtin_macros::with_builtin;

    pub use crate::{
        arguments::{Arguments, StdFmt, StdFmtDebug, StdFmtOther},
        formatter::{Align, DebugHex, Formatter, FormatterArgs, Sign},
        Background, Color, Display, Foreground, Intensity, StyleDiff,
    };
}

#[cfg(feature = "macros")]
/// Constructs parameters for the other string-formatting macros.
///
/// This macro functions by taking a formatting string literal containing `{}`
/// for each additional argument passed. `format_args!` prepares the additional
/// parameters to ensure the output can be interpreted as a string and
/// canonicalizes the arguments into a single type. Any value that implements
/// the [`stylish::Display`] trait or any of the [`std::fmt`] formatting traits
/// can be passed to `format_args!` with the appropriate trait selector.
///
/// This macro produces a value of type [`stylish::Arguments`]. This value can
/// be passed to the macros within [`stylish`] for performing useful
/// redirection. All other formatting macros ([`stylish::format!`],
/// [`stylish::write!`], etc) are proxied through this one. `format_args!`,
/// unlike some of its derived macros, avoids heap allocations.
///
/// For more information, see the documentation in [`stylish`].
///
/// # Examples
///
/// ```rust
/// let s = stylish::html::format(stylish::format_args!(
///     "hello {:(fg=green)}",
///     "world"
/// ));
/// assert_eq!(s, stylish::html::format!("hello {:(fg=green)}", "world"));
/// ```
#[macro_export]
macro_rules! format_args {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::_format_args!($fmt $(, $($arg)*)?)
    };
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::_format_args!($fmt $(, $($arg)*)?)
    };
}

#[cfg(feature = "macros")]
#[cfg(not(stylish_proc_macro_expand))]
#[doc(hidden)]
#[macro_export]
macro_rules! _format_args {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::𓀄::format_args!(crate=$crate, $fmt $(, $($arg)*)?)
    };
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::𓀄::with_builtin!(let $fmt_lit = $fmt in {
            $crate::𓀄::format_args!(crate=$crate, $fmt_lit $(, $($arg)*)?)
        })
    };
}

#[cfg(feature = "macros")]
#[cfg(stylish_proc_macro_expand)]
#[doc(hidden)]
#[macro_export]
macro_rules! _format_args {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::𓀄::format_args!(crate=$crate, $fmt $(, $($arg)*)?)
    };
}
