use crate::format;
use alloc::string::String;
use stylish_core::Display;

pub trait ToAnsiString {
    fn to_ansi_string(&self) -> String;
}

impl<T> ToAnsiString for T
where
    T: Display + ?Sized,
{
    fn to_ansi_string(&self) -> String {
        format!("{:s}", self)
    }
}
