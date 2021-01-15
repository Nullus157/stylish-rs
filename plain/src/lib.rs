#![feature(decl_macro, rustc_attrs)]

use stylish_core::Style;

#[doc(hidden)]
pub mod __export {
    pub use stylish_core::format_args;
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = $crate::format($crate::__export::format_args!($($arg)*));
        res
    }}
}

pub fn format(args: stylish_core::Arguments<'_>) -> String {
    use stylish_core::Write;
    let mut string = String::new();
    Plain::new(&mut string).write_fmt(args).unwrap();
    string
}

#[derive(Clone, Debug, Default)]
pub struct Plain<T> {
    inner: T,
}

impl<T> Plain<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: std::fmt::Write> stylish_core::Write for Plain<T> {
    fn write_str(&mut self, s: &str, _style: Style) -> std::fmt::Result {
        self.inner.write_str(s)
    }
}

impl<T: std::io::Write> stylish_core::io::Write for Plain<T> {
    fn write(&mut self, s: &[u8], _style: Style) -> std::io::Result<usize> {
        self.inner.write(s)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, s: &[u8], _style: Style) -> std::io::Result<()> {
        self.inner.write_all(s)
    }
}
