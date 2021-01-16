#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Intensity {
    Normal,
    Bold,
    Faint,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub intensity: Intensity,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Foreground(pub Color);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Background(pub Color);

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

impl Restyle for Foreground {
    fn apply(&self, style: Style) -> Style {
        let &Foreground(foreground) = self;
        Style {
            foreground,
            ..style
        }
    }
}

impl Restyle for Background {
    fn apply(&self, style: Style) -> Style {
        let &Background(background) = self;
        Style {
            background,
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
