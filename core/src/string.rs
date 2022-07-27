use crate::{Display, Formatter, Result, Style, Write};

/// An attributed version of [`alloc::string::String`] which has a specific
/// [`Style`] associated with each character.
///
/// The main interfaces to create an instance of this are [`stylish::format!`]
/// and [`impl Write for String`](#impl-Write), and to inspect the content
/// [`impl Display for String`](#impl-Display).
///
/// You can create an instance via [`stylish::format!`]:
///
/// ```rust
/// let _: stylish::String = stylish::format!("Hello {:(fg=green)}!", "World");
/// ```
///
/// You can append extra data via [`stylish::write!`]:
///
/// ```rust
/// use stylish::Write;
///
/// let mut s = stylish::String::new();
/// stylish::write!(s, "{:(fg=magenta)}", "fuchsia")?;
/// # Ok::<(), std::fmt::Error>(())
/// ```
///
/// To use the attributed data you must then write this string to a sink using
/// the `{:s}` trait-selector.
///
/// ```rust
/// use stylish::Write;
///
/// let mut s = stylish::format!("Hello {:(fg=green)}!", "World");
/// stylish::write!(s, " Is it {:(fg=magenta)} or?", "fuchsia")?;
///
/// assert_eq!(
///     stylish::html::format!("{:s}", s),
///     "Hello <span style=color:green>World</span>! \
///     Is it <span style=color:magenta>fuchsia</span> or?",
/// );
/// # Ok::<(), std::fmt::Error>(())
/// ```
#[derive(Default, Debug, Clone)]
pub struct String {
    string: alloc::string::String,
    styles: alloc::vec::Vec<(usize, Style)>,
}

impl String {
    /// Create an empty [`String`].
    ///
    /// ```rust
    /// assert_eq!(stylish::html::format!("{:s}", stylish::String::new()), "");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl Write for String {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        if Some(style) != self.styles.last().map(|&(_, style)| style) {
            self.styles.push((self.string.len(), style));
        }
        self.string.push_str(s);
        Ok(())
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut styles = self.styles.iter().peekable();
        while let Some(&(start, style)) = styles.next() {
            let end = styles
                .peek()
                .map(|&&(end, _)| end)
                .unwrap_or_else(|| self.string.len());
            f.with(style).write_str(&self.string[start..end])?;
        }
        Ok(())
    }
}
