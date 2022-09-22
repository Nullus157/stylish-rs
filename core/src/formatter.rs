use crate::{Arguments, Display, Restyle, Result, Style, Write};

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[derive(Clone, Copy, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[derive(Clone, Copy, Debug)]
pub enum DebugHex {
    Lower,
    Upper,
}

#[doc(hidden)] // workaround https://github.com/rust-lang/rust/issues/85522
#[derive(Clone, Copy, Debug, Default)]
pub struct FormatterArgs<'a> {
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero: bool,
    pub width: Option<&'a usize>,
    pub precision: Option<&'a usize>,
    pub debug_hex: Option<DebugHex>,
}

/// A configured output stream.
///
/// A `Formatter` wraps a target output stream with a set of configuration
/// options for formatting of data written to the stream. There is (currently)
/// no public constructors for `Formatter`, an instance is created and passed to
/// implementations of [`stylish::Display`] when they are used in
/// the [`stylish`] macros.
pub struct Formatter<'a> {
    style: Style,
    pub(crate) format: FormatterArgs<'a>,
    write: &'a mut (dyn Write + 'a),
}

impl core::fmt::Debug for Formatter<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        f.debug_struct("Formatter")
            .field("style", &self.style)
            .field("format", &self.format)
            .finish()
    }
}

impl<'a> Formatter<'a> {
    #[inline]
    pub(crate) fn new(write: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            style: Style::default(),
            format: FormatterArgs::default(),
            write,
        }
    }

    // TODO: All the rest of the std::fmt::Formatter methods

    /// Create a sub-`Formatter` with some styles changed. This may be useful in
    /// implementations of [`stylish::Display`] to dynamically configure how
    /// some parts are formatted.
    ///
    /// ```rust
    /// struct Name(&'static str);
    ///
    /// impl stylish::Display for Name {
    ///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
    ///         let color = match self.0 {
    ///             "Ferris" => stylish::Color::Red,
    ///             "Gorris" => stylish::Color::Cyan,
    ///             _ => stylish::Color::Default,
    ///         };
    ///         f.with(stylish::Foreground(color)).write_str(self.0)
    ///     }
    /// }
    ///
    /// let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
    /// assert_eq!(
    ///     formatted,
    ///     "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>"
    /// );
    /// ```
    pub fn with(&mut self, restyle: impl Restyle) -> Formatter<'_> {
        Formatter {
            write: &mut *self.write,
            format: self.format,
            style: self.style.with(restyle),
        }
    }

    #[doc(hidden)]
    /// pub for macros
    pub fn with_args<'b>(
        &'b mut self,
        format: &FormatterArgs<'b>,
        restyle: impl Restyle,
    ) -> Formatter<'b> {
        Formatter {
            write: &mut *self.write,
            format: *format,
            style: self.style.with(restyle),
        }
    }

    /// Writes some data to the underlying output stream, using the current
    /// style.
    ///
    /// ```rust
    /// struct Name(&'static str);
    ///
    /// impl stylish::Display for Name {
    ///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
    ///         let color = match self.0 {
    ///             "Ferris" => stylish::Color::Red,
    ///             "Gorris" => stylish::Color::Cyan,
    ///             _ => stylish::Color::Default,
    ///         };
    ///         f.with(stylish::Foreground(color)).write_str(self.0)
    ///     }
    /// }
    ///
    /// let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
    /// assert_eq!(
    ///     formatted,
    ///     "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>"
    /// );
    /// ```
    #[inline]
    pub fn write_str(&mut self, s: &str) -> Result {
        self.write.write_str(s, self.style)?;
        Ok(())
    }

    /// Writes some formatted data into this instance, overriding the current
    /// style as appropriate.
    ///
    /// ```rust
    /// struct Name(&'static str);
    ///
    /// impl stylish::Display for Name {
    ///     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
    ///         match self.0 {
    ///             "Ferris" => f.write_fmt(stylish::format_args!("{:(fg=red)}", self.0)),
    ///             "Gorris" => f.write_fmt(stylish::format_args!("{:(fg=cyan)}", self.0)),
    ///             _ => f.write_fmt(stylish::format_args!("{}", self.0)),
    ///         }
    ///     }
    /// }
    ///
    /// let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
    /// assert_eq!(
    ///     formatted,
    ///     "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>"
    /// );
    /// ```
    #[inline]
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        args.fmt(self)?;
        Ok(())
    }
}

impl<'a> Write for Formatter<'a> {
    #[inline]
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        self.with(style).write_str(s)
    }

    #[inline]
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        self.write_fmt(args)
    }
}

impl<'a> core::fmt::Write for Formatter<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.write_str(s)
    }
}
