#![no_std]

//! The semver-stable subset of [`stylish`].
//!
//! See the main documentation in [`stylish`], there is no reason to depend on
//! this crate directly.

#![warn(missing_docs)]

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
mod style;
#[cfg(feature = "alloc")]
mod to_string;
mod write;

pub use self::{
    arguments::Arguments,
    display::Display,
    formatter::Formatter,
    style::{Background, Color, Foreground, Intensity, Restyle, Style, StyleDiff},
    write::Write,
};
#[cfg(feature = "alloc")]
pub use self::{format::format, string::String, to_string::ToStylishString};
pub use core::fmt::{Error, Result};

#[doc(hidden)]
pub mod __export {
    pub use crate::{
        arguments::{Argument, Arguments, FormatTrait, StdFmt},
        formatter::{Align, DebugHex, FormatterArgs, Sign},
        Background, Color, Display, Foreground, Intensity,
    };
    pub use core::{fmt, option::Option};
    pub use stylish_macros;
    pub use with_builtin_macros::with_builtin;
}

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
/// let s = stylish::html::format(stylish::format_args!("hello {:(fg=green)}", "world"));
/// assert_eq!(s, stylish::html::format!("hello {:(fg=green)}", "world"));
/// ```
#[macro_export]
macro_rules! format_args {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::__export::stylish_macros::format_args!(crate=$crate, $fmt $(, $($arg)*)?)
    };
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::__export::with_builtin!(let $fmt_lit = $fmt in {
            $crate::__export::stylish_macros::format_args!(crate=$crate, $fmt_lit $(, $($arg)*)?)
        })
    };
}
