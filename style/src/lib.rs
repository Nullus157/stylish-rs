#![no_std]

//! Internal implementation details of [`stylish-core`](https://docs.rs/stylish-core).
//!
//! Do not depend on this crate directly.

#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// A color that can be used with [`Foreground`] to modify [`Style::foreground`]
/// or [`Background`] to modify [`Style::background`].
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Color {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    // Bright Black
    BrightBlack,
    // Bright Red
    BrightRed,
    // Bright Green
    BrightGreen,
    // Bright Yellow
    BrightYellow,
    // Bright Blue
    BrightBlue,
    // Bright Magenta
    BrightMagenta,
    // Bright Cyan
    BrightCyan,
    // Bright White
    BrightWhite,
    /// Default color
    Default,
}

/// An intensity to render text with, to emphasise or de-emphasise it as needed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Intensity {
    /// The normal intensity
    Normal,
    /// A bolder intensity to emphasise content
    Bold,
    /// A fainter intensity to de-emphasise content
    Faint,
}

/// A style to render text with, setting the foreground and background colors,
/// along with intensity.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Style {
    /// The text foreground color
    pub foreground: Color,
    /// The text background color
    pub background: Color,
    /// The text intensity
    pub intensity: Intensity,
}

/// A diff between two styles.
///
/// Most useful for some implementors of `stylish::Write` to detect changes
/// between two parts, or for applying multiple changes to a style at once.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct StyleDiff {
    /// The change in the text foreground color
    pub foreground: Option<Color>,
    /// The change in the text background color
    pub background: Option<Color>,
    /// The change in the text intensity
    pub intensity: Option<Intensity>,
}

/// A [`Restyle`] implementor for setting [`Style::foreground`].
///
/// ```rust
/// use stylish::{Color, Foreground, Style};
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
/// use stylish::{Background, Color, Style};
///
/// let mut expected = Style::default();
/// expected.background = Color::Magenta;
///
/// assert_eq!(Style::default().with(Background(Color::Magenta)), expected);
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Background(pub Color);

/// A trait for modifications to [`Style`], allowing an ergonomic API with
/// [`Style::with`] and `stylish::Formatter::with`.
///
/// ```rust
/// use stylish::{Color, Foreground, Intensity, Restyle, Style};
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
    /// Apply the restyling this instance represents to an existing style,
    /// returning the updated style.
    fn apply(&self, style: Style) -> Style;
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

impl Default for Intensity {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

impl Style {
    /// Apply a modification to this style, returning the result.
    ///
    /// ```rust
    /// use stylish::{Intensity, Style};
    ///
    /// let mut expected = Style::default();
    /// expected.intensity = Intensity::Faint;
    ///
    /// assert_eq!(Style::default().with(Intensity::Faint), expected);
    /// ```
    pub fn with(self, adj: impl Restyle) -> Self {
        adj.apply(self)
    }

    /// Find the changes from the `original` style that would result in this
    /// style.
    ///
    /// This can be useful for writers like `ansi` that are stateful, finding
    /// the minimal state that must be changed between the current output
    /// style and the new style.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, StyleDiff};
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
    ///     }
    /// ));
    /// ```
    #[inline]
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
        (**self).apply(style)
    }
}

#[cfg(feature = "alloc")]
impl<T: Restyle + ?Sized> Restyle for alloc::boxed::Box<T> {
    fn apply(&self, style: Style) -> Style {
        (**self).apply(style)
    }
}

impl<T: Restyle> Restyle for [T] {
    fn apply(&self, mut style: Style) -> Style {
        for restyle in self {
            style = restyle.apply(style);
        }
        style
    }
}

impl<T: Restyle> Restyle for Option<T> {
    fn apply(&self, style: Style) -> Style {
        self.as_ref().map_or(style, |s| s.apply(style))
    }
}

impl Restyle for StyleDiff {
    #[inline]
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
    #[inline]
    fn apply(&self, _style: Style) -> Style {
        *self
    }
}

impl Restyle for Foreground {
    #[inline]
    fn apply(&self, style: Style) -> Style {
        let &Foreground(foreground) = self;
        Style {
            foreground,
            ..style
        }
    }
}

impl Restyle for Background {
    #[inline]
    fn apply(&self, style: Style) -> Style {
        let &Background(background) = self;
        Style {
            background,
            ..style
        }
    }
}

impl Restyle for Intensity {
    #[inline]
    fn apply(&self, style: Style) -> Style {
        Style {
            intensity: *self,
            ..style
        }
    }
}

impl Restyle for () {
    #[inline]
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
