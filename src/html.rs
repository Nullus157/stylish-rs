use askama_escape::{escape, Html};
use stylish::{Color, Intensity, Style};

#[derive(Clone, Debug)]
pub struct String {
    inner: std::string::String,
    current: Style,
}

impl String {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(mut self) -> std::string::String {
        self.inner.push_str("</span>");
        self.inner
    }
}

impl Default for String {
    fn default() -> Self {
        Self {
            inner: std::string::String::from("<span>"),
            current: Style::default(),
        }
    }
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

impl stylish::Write for String {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str, style: Style) -> std::fmt::Result {
        use std::fmt::Write;

        if style != self.current {
            write!(
                self.inner,
                r#"</span><span style="color: {}; font-weight: {}">"#,
                color(style.color),
                intensity(style.intensity),
            )?;
            self.current = style;
        }
        write!(self.inner, "{}", escape(s, Html))?;
        Ok(())
    }
}
