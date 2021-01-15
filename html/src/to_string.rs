use crate::format;
use stylish_core::Display;

pub trait ToHtmlString {
    fn to_html_string(&self) -> String;
}

impl<T> ToHtmlString for T
where
    T: Display + ?Sized,
{
    fn to_html_string(&self) -> String {
        format!("{:s}", self)
    }
}
