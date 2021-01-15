use askama_escape::{escape, Html as AskamaHtml};
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
    let mut html = Html::new(String::new());
    html.write_fmt(&args).unwrap();
    html.finish_fmt().unwrap()
}

fn color(color: Color) -> &'static str {
    match color {
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::White => "white",
        Color::Default => "inherit",
    }
}

fn intensity(intensity: Intensity) -> &'static str {
    match intensity {
        Intensity::Normal => "inherit",
        Intensity::Bold => "bolder",
        Intensity::Faint => "lighter",
    }
}

#[derive(Clone, Debug, Default)]
pub struct Html<T> {
    inner: Option<T>,
    current: Option<Style>,
}

impl<T> Drop for Html<T> {
    fn drop(&mut self) {
        if self.current.is_some() {
            panic!("Dropped stylish::Html without finishing it");
        }
    }
}

impl<T> Html<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            current: None,
        }
    }
}

impl<T: std::fmt::Write> Html<T> {
    pub fn finish_fmt(mut self) -> Result<T, std::fmt::Error> {
        if self.current.is_some() {
            write!(self.inner.as_mut().unwrap(), "</span>")?;
        }
        Ok(self.inner.take().unwrap())
    }
}

impl<T: std::io::Write> Html<T> {
    pub fn finish_io(mut self) -> std::io::Result<T> {
        if self.current.is_some() {
            write!(self.inner.as_mut().unwrap(), "</span>")?;
        }
        Ok(self.inner.take().unwrap())
    }
}

impl<T: std::fmt::Write> stylish_core::Write for Html<T> {
    fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
        if Some(style) != self.current {
            if self.current.is_some() {
                write!(self.inner.as_mut().unwrap(), "</span>")?;
            }
            write!(
                self.inner.as_mut().unwrap(),
                r#"<span style="color: {}; font-weight: {}">"#,
                color(style.color),
                intensity(style.intensity),
            )?;
            self.current = Some(style);
        }
        write!(self.inner.as_mut().unwrap(), "{}", escape(s, AskamaHtml))?;
        Ok(())
    }
}

impl<T: std::io::Write> stylish_core::io::Write for Html<T> {
    fn write_str(&mut self, s: &str, style: Style) -> std::io::Result<()> {
        if Some(style) != self.current {
            if self.current.is_some() {
                write!(self.inner.as_mut().unwrap(), "</span>")?;
            }
            write!(
                self.inner.as_mut().unwrap(),
                r#"<span style="color: {}; font-weight: {}">"#,
                color(style.color),
                intensity(style.intensity),
            )?;
            self.current = Some(style);
        }
        write!(self.inner.as_mut().unwrap(), "{}", escape(s, AskamaHtml))?;
        Ok(())
    }
}
