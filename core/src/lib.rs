#![no_std]

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
    style::{Color, Intensity, Restyle, Style},
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
        Display,
    };
    pub use core::{fmt, option::Option};
    pub use stylish_macros;
}

#[macro_export]
macro_rules! format_args {
    ($($arg:tt)*) => {
        $crate::__export::stylish_macros::format_args!(crate=$crate, $($arg)*)
    };
}
