use stylish_core::{Color, Intensity};

pub(crate) fn foreground(color: Color) -> &'static str {
    match color {
        Color::Black => "color:black",
        Color::Red => "color:red",
        Color::Green => "color:green",
        Color::Yellow => "color:yellow",
        Color::Blue => "color:blue",
        Color::Magenta => "color:magenta",
        Color::Cyan => "color:cyan",
        Color::White => "color:white",
        Color::Default => "color:inherit",
        _ => "color:inherit",
    }
}

pub(crate) fn background(color: Color) -> &'static str {
    match color {
        Color::Black => "background-color:black",
        Color::Red => "background-color:red",
        Color::Green => "background-color:green",
        Color::Yellow => "background-color:yellow",
        Color::Blue => "background-color:blue",
        Color::Magenta => "background-color:magenta",
        Color::Cyan => "background-color:cyan",
        Color::White => "background-color:white",
        Color::Default => "background-color:inherit",
        _ => "background-color:inherit",
    }
}

pub(crate) fn intensity(intensity: Intensity) -> &'static str {
    match intensity {
        Intensity::Bold => "font-weight:bolder",
        Intensity::Faint => "font-weight:lighter",
        Intensity::Normal => "font-weight:inherit",
        _ => "font-weight:inherit",
    }
}
