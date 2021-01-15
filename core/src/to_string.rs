use crate::{format, Display, String};

pub trait ToStylishString {
    fn to_stylish_string(&self) -> String;
}

impl<T> ToStylishString for T
where
    T: Display + ?Sized,
{
    fn to_stylish_string(&self) -> String {
        format!("{:s}", self)
    }
}
