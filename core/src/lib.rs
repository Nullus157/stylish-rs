pub mod io;

#[macro_use]
mod std_compat;

mod arguments;
mod display;
mod format;
mod formatter;
mod string;
mod style;
mod to_string;
mod write;

pub use self::{
    arguments::Arguments,
    display::Display,
    format::format,
    formatter::Formatter,
    string::String,
    style::{Color, Intensity, Restyle, Style},
    to_string::ToStylishString,
    write::Write,
};
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
