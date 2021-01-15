pub mod io;

#[macro_use]
mod std_compat;

mod arguments;
mod display;
mod formatter;
mod style;
mod write;

pub use self::{
    arguments::Arguments,
    display::Display,
    formatter::Formatter,
    style::{Color, Intensity, Restyle, Style},
    write::Write,
};
pub use core::fmt::{Error, Result};

#[doc(hidden)]
pub mod __export {
    pub use crate::{
        arguments::{Argument, Arguments, FormatTrait, StdFmt},
        formatter::{Align, DebugHex, FormatterArgs, Sign},
        Display,
    };
    pub use core::{fmt, option::Option};
    pub use stylish_macros::{format_args, format_args_nl};
}

#[macro_export]
macro_rules! format_args {
    ($($arg:tt)*) => {
        $crate::__export::format_args!(crate=$crate, $($arg)*)
    };
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(&$crate::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::__export::format_args_nl!(crate=$crate, $($arg)*))
    };
}
