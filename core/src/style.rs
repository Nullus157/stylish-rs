/// A color that can be used with [`Foreground`] to modify [`Style::foreground`] or [`Background`]
/// to modify [`Style::background`].
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

/// An intensity to render text with, to emphasise or de-emphasise it as needed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Intensity {
    Normal,
    Bold,
    Faint,
}

/// A style to render text with, setting the foreground and background colors, along with intensity.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub intensity: Intensity,
}

/// A diff between two styles.
///
/// Most useful for some implementors of [`stylish::Write`] to detect changes between two parts, or
/// for applying multiple changes to a style at once.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct StyleDiff {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub intensity: Option<Intensity>,
}

/// A [`Restyle`] implementor for setting [`Style::foreground`].
///
/// ```rust
/// use stylish::{Style, Foreground, Color};
///
/// let mut expected = Style::default();
/// expected.foreground = Color::Magenta;
///
/// assert_eq!(Style::default().with(Foreground(Color::Magenta)), expected);
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Foreground(pub Color);

/// A [`Restyle`] implementor for setting [`Style::background`].
///
/// ```rust
/// use stylish::{Style, Color, Background};
///
/// let mut expected = Style::default();
/// expected.background = Color::Magenta;
///
/// assert_eq!(Style::default().with(Background(Color::Magenta)), expected);
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Background(pub Color);

/// A trait for modifications to [`Style`], allowing an ergonomic API with [`Style::with`] and
/// [`Formatter::with`].
///
/// ```rust
/// use stylish::{Style, Foreground, Restyle, Intensity, Color};
///
/// struct OhNo;
///
/// impl Restyle for OhNo {
///     fn apply(&self, style: Style) -> Style {
///         style.with(Foreground(Color::Red)).with(Intensity::Bold)
///     }
/// }
///
/// let mut expected = Style::default();
/// expected.foreground = Color::Red;
/// expected.intensity = Intensity::Bold;
///
/// assert_eq!(Style::default().with(OhNo), expected);
/// ```
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
    /// Apply a modification to this style, returning the result.
    ///
    /// ```rust
    /// use stylish::{Style, Intensity};
    ///
    /// let mut expected = Style::default();
    /// expected.intensity = Intensity::Faint;
    ///
    /// assert_eq!(Style::default().with(Intensity::Faint), expected);
    /// ```
    pub fn with(self, adj: impl Restyle) -> Self {
        adj.apply(self)
    }

    /// Find the changes from the `original` style that would result in this style.
    ///
    /// This can be useful for writers like `ansi` that are stateful, finding the minimal state
    /// that must be changed between the current output style and the new style.
    ///
    /// ```rust
    /// use stylish::{Style, StyleDiff, Foreground, Color};
    ///
    /// let original = Style::default();
    /// let updated = original.with(Foreground(Color::Cyan));
    ///
    /// assert!(matches!(
    ///     updated.diff_from(original),
    ///     StyleDiff {
    ///         foreground: Some(Color::Cyan),
    ///         background: None,
    ///         intensity: None,
    ///         ..
    ///     }));
    /// ```
    pub fn diff_from(self, original: Style) -> StyleDiff {
        fn diff<T: PartialEq>(original: T, new: T) -> Option<T> {
            if original == new {
                None
            } else {
                Some(new)
            }
        }

        StyleDiff {
            foreground: diff(original.foreground, self.foreground),
            background: diff(original.background, self.background),
            intensity: diff(original.intensity, self.intensity),
        }
    }
}

impl<T: Restyle + ?Sized> Restyle for &T {
    fn apply(&self, style: Style) -> Style {
        (&**self).apply(style)
    }
}

impl<T: Restyle> Restyle for Option<T> {
    fn apply(&self, style: Style) -> Style {
        self.as_ref().map_or(style, |s| s.apply(style))
    }
}

impl Restyle for StyleDiff {
    fn apply(&self, style: Style) -> Style {
        (
            self.foreground.map(Foreground),
            self.background.map(Background),
            self.intensity,
        )
            .apply(style)
    }
}

impl Restyle for Style {
    fn apply(&self, _style: Style) -> Style {
        *self
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

impl<T: Restyle, U: Restyle, V: Restyle> Restyle for (T, U, V) {
    fn apply(&self, style: Style) -> Style {
        style.with(&self.0).with(&self.1).with(&self.2)
    }
}
