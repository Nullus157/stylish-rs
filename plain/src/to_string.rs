use crate::format;
use stylish_core::Display;

pub trait ToPlainString {
    fn to_plain_string(&self) -> String;
}

impl<T> ToPlainString for T
where
    T: Display + ?Sized,
{
    fn to_plain_string(&self) -> String {
        format!("{:s}", self)
    }
}
