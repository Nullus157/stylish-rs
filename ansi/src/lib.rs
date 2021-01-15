use stylish_core::{Color, Intensity, Style};

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
    let mut ansi = Ansi::new(String::new());
    ansi.write_fmt(args).unwrap();
    ansi.finish().unwrap()
}

fn color(color: Color) -> u8 {
    match color {
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::Default => 39,
    }
}

fn intensity(intensity: Intensity) -> u8 {
    match intensity {
        Intensity::Normal => 22,
        Intensity::Bold => 1,
        Intensity::Faint => 2,
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ansi<T: std::fmt::Write> {
    inner: Option<T>,
    current: Option<Style>,
}

impl<T: std::fmt::Write> Drop for Ansi<T> {
    fn drop(&mut self) {
        if self.current.is_some() {
            panic!("Dropped stylish::Ansi without finishing it");
        }
    }
}

impl<T: std::fmt::Write> Ansi<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            current: None,
        }
    }

    pub fn finish(mut self) -> Result<T, std::fmt::Error> {
        if self.current.is_some() {
            write!(self.inner.as_mut().unwrap(), "[0m")?;
        }
        Ok(self.inner.take().unwrap())
    }
}

impl<T: std::fmt::Write> stylish_core::Write for Ansi<T> {
    fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
        if Some(style) != self.current {
            write!(
                self.inner.as_mut().unwrap(),
                "[{};{}m",
                color(style.color),
                intensity(style.intensity),
            )?;
            self.current = Some(style);
        }
        write!(self.inner.as_mut().unwrap(), "{}", s)?;
        Ok(())
    }
}
