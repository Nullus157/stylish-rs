use stylish_core::{Color, Intensity};

pub(crate) fn color(color: Color) -> u8 {
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

pub(crate) fn intensity(intensity: Intensity) -> u8 {
    match intensity {
        Intensity::Normal => 22,
        Intensity::Bold => 1,
        Intensity::Faint => 2,
    }
}
