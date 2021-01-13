#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(associated_type_bounds)]

extern crate self as stylish;

pub mod ansi;
pub mod html;
pub mod plain;
pub mod style;

#[doc(hidden)]
pub mod arguments;
mod formatter;
mod traits;
mod write;

pub use stylish::{
    arguments::{Argument, Arguments},
    formatter::{Align, DebugHex, Formatter, FormatterArgs, Sign},
    style::{Color, Intensity, Style},
    traits::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
};

pub mod fmt {
    pub use crate::{
        write::fmt::Write, Arguments, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal,
        Pointer, UpperExp, UpperHex,
    };
    pub use core::fmt::{Error, Result};
    pub use stylish_macros::{format_args, write, writeln};
}

pub mod io {
    pub use crate::write::io::Write;
    pub use std::io::{Error, Result};
}

pub use stylish_macros::{format_args, write, writeln};
