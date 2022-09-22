use crate::{Arguments, Formatter, Result, Style};

/// A trait for writing or formatting into attributed Unicode-accepting buffers
/// or streams.
///
///
///
/// This trait only accepts UTF-8–encoded data and is not
/// [flushable](stylish::io::Write::flush). If you only want to accept Unicode
/// and you don’t need flushing, you should implement this trait; otherwise you
/// should implement [`stylish::io::Write`].
pub trait Write {
    /// Writes a string slice with a particular [`Style`] into this writer,
    /// returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully
    /// written, and this method will not return until all data has been
    /// written or an error occurs.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, Write};
    ///
    /// let blue = Style::default().with(Foreground(Color::Blue));
    ///
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_str("water is ", Style::default())?;
    ///     output.write_str("blue", blue)?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(s, "water is <span style=color:blue>blue</span>");
    /// # Ok::<(), core::fmt::Error>(())
    /// ```
    fn write_str(&mut self, s: &str, style: Style) -> Result;

    /// Writes a [`char`] with a particular [`Style`] into this writer,
    /// returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte. This method can
    /// only succeed if the entire byte sequence was successfully written,
    /// and this method will not return until all data has been written or
    /// an error occurs.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, Write};
    ///
    /// let yellow = Style::default().with(Foreground(Color::Yellow));
    /// let red = Style::default().with(Foreground(Color::Red));
    ///
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_char('⚠', yellow)?;
    ///     output.write_char(' ', Style::default())?;
    ///     output.write_char('⛔', red)?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(
    ///     s,
    ///     "<span style=color:yellow>⚠</span> <span style=color:red>⛔</span>"
    /// );
    /// # Ok::<(), core::fmt::Error>(())
    /// ```
    #[inline]
    fn write_char(&mut self, c: char, style: Style) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]), style)
    }

    /// Glue for usage of the [`stylish::write!`] macro with implementors of
    /// this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`stylish::write!`] macro itself.
    ///
    /// ```rust
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_fmt(stylish::format_args!("{:(fg=red)}", '☎'))?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(s, "<span style=color:red>☎</span>");
    /// # Ok::<(), core::fmt::Error>(())
    /// ```
    #[inline]
    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        Formatter::new(&mut self).write_fmt(args)
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (**self).write_str(s, style)
    }

    fn write_char(&mut self, c: char, style: Style) -> Result {
        (**self).write_char(c, style)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        (**self).write_fmt(args)
    }
}

/// Writes attributed and formatted data into a buffer.
///
/// This macro accepts a 'writer', a format string, and a list of arguments.
/// Arguments will be formatted according to the specified format string and the
/// result will be passed to the writer. The writer may be any value with a
/// `write_fmt` method of the right signature; generally this comes from an
/// implementation of either the [`stylish::Write`] or the
/// [`stylish::io::Write`] trait. The macro returns whatever the `write_fmt`
/// method returns; commonly a [`core::fmt::Result`], or a [`std::io::Result`].
///
/// See [`stylish`] for more information on the format string syntax.
///
/// # Examples
///
/// ```rust
/// let mut w = stylish::html(String::new());
///
/// stylish::write!(&mut w, "test")?;
/// stylish::write!(&mut w, "formatted {:(fg=yellow)}", "arguments")?;
///
/// assert_eq!(
///     w.finish()?,
///     "testformatted <span style=color:yellow>arguments</span>"
/// );
/// # Ok::<(), core::fmt::Error>(())
/// ```
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// Write attributed and formatted data into a buffer, with a newline appended.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`)
/// alone (no additional CARRIAGE RETURN (`\r`/`U+000D`).
///
/// For more information, see [`stylish::write!`]. For information on the format
/// string syntax, see [`stylish`].
///
/// # Examples
///
/// ```rust
/// let mut w = stylish::html(String::new());
///
/// stylish::writeln!(&mut w)?;
/// stylish::writeln!(&mut w, "test")?;
/// stylish::writeln!(&mut w, "formatted {:(fg=yellow)}", "arguments")?;
///
/// assert_eq!(w.finish()?, "\ntest\nformatted <span style=color:yellow>arguments</span>\n");
/// # Ok::<(), core::fmt::Error>(())
#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::𓀄::format_args_nl!(crate=$crate, $($arg)*))
    };
}
