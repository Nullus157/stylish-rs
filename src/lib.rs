#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(associated_type_bounds)]

extern crate self as stylish;

pub mod ansi;
pub mod html;
pub mod plain;
pub mod style;

mod arguments;
mod display;
mod formatter;
mod write;

pub use stylish::{
    arguments::{Argument, Arguments},
    display::Display,
    formatter::Formatter,
    style::{Color, Intensity, Style},
    write::Write,
};
pub use stylish_macros::{format_args, write, writeln};
