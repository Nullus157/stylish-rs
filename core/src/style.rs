#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    Default = 39,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Intensity {
    Normal = 22,
    Bold = 1,
    Faint = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Style {
    pub color: Color,
    pub intensity: Intensity,
}

pub trait Restyle {
    fn apply(&self, style: Style) -> Style;
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

impl Default for Intensity {
    fn default() -> Self {
        Self::Normal
    }
}

impl Style {
    pub fn with(self, adj: impl Restyle) -> Self {
        adj.apply(self)
    }
}

impl<T: Restyle + ?Sized> Restyle for &T {
    fn apply(&self, style: Style) -> Style {
        (&**self).apply(style)
    }
}

impl Restyle for Style {
    fn apply(&self, style: Style) -> Style {
        style
    }
}

impl Restyle for Color {
    fn apply(&self, style: Style) -> Style {
        Style {
            color: *self,
            ..style
        }
    }
}

impl Restyle for Intensity {
    fn apply(&self, style: Style) -> Style {
        Style {
            intensity: *self,
            ..style
        }
    }
}

impl Restyle for () {
    fn apply(&self, style: Style) -> Style {
        style
    }
}

impl<T: Restyle, U: Restyle> Restyle for (T, U) {
    fn apply(&self, style: Style) -> Style {
        style.with(&self.0).with(&self.1)
    }
}
