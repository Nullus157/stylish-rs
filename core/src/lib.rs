#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(crate_visibility_modifier)]
#![feature(associated_type_bounds)]

pub mod io;
pub mod style;

mod arguments;
mod formatter;
mod traits;
mod write;

pub use self::{
    arguments::Arguments,
    formatter::Formatter,
    style::{Color, Intensity, Style},
    traits::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
    write::Write,
};
pub use core::fmt::{Error, Result};

#[doc(hidden)]
pub mod __export {
    pub use crate::{
        arguments::{arg, Argument},
        formatter::{Align, DebugHex, FormatterArgs, Sign},
        Arguments, Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
    };
    pub use core::option::Option;
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
