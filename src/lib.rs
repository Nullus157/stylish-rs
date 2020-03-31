extern crate self as stylish;

pub mod ansi;
pub mod plain;
pub mod style;
pub mod html;

mod arguments;
mod formatter;
mod display;
mod write;

pub use self::{style::{Color, Intensity, Style}, formatter::Formatter, display::Display, write::Write, arguments::{Arguments, Argument}};
