#![no_std]
#![feature(extended_key_value_attributes)]

#![doc = crate::__export::docs::krate!()]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub mod io;

#[cfg(doc)]
extern crate self as stylish;

#[cfg(all(doc, not(feature = "std")))]
use core::fmt as doc_fmt;

#[cfg(all(doc, feature = "std"))]
use std::fmt as doc_fmt;

#[macro_use]
mod std_compat;

mod arguments;
mod display;
mod docs;
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
    style::{Background, Color, Foreground, Intensity, Restyle, Style},
    write::Write,
};
#[cfg(feature = "alloc")]
pub use self::{format::format, string::String, to_string::ToStylishString};
pub use core::fmt::Error;

pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[doc(hidden)]
pub mod __export {
    pub use crate::{
        arguments::{Argument, Arguments, FormatTrait, StdFmt},
        formatter::{Align, DebugHex, FormatterArgs, Sign},
        style::{Background, Color, Foreground, Intensity},
        Display,
    };
    pub mod docs {
        pub use crate::__docs_display_example as display_example;
        pub use crate::__docs_crate as krate;
    }
    pub use core::{fmt, option::Option};
    pub use stylish_macros;
    pub use with_builtin_macros::with_builtin;
}

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
