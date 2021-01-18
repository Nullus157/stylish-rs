use stylish_core::{Color, Intensity};

pub(crate) fn foreground(color: Color) -> &'static str {
    match color {
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
        Color::Default => "39",
        _ => "39",
    }
}

pub(crate) fn background(color: Color) -> &'static str {
    match color {
        Color::Black => "40",
        Color::Red => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta => "45",
        Color::Cyan => "46",
        Color::White => "47",
        Color::Default => "49",
        _ => "49",
    }
}

pub(crate) fn intensity(intensity: Intensity) -> &'static str {
    match intensity {
        Intensity::Bold => "1",
        Intensity::Faint => "2",
        Intensity::Normal => "22",
        _ => "22",
    }
}
