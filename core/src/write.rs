use crate::{Arguments, Formatter, Result, Style};

/// A trait for writing or formatting into attributed Unicode-accepting buffers or streams.
///
///
///
/// This trait only accepts UTF-8–encoded data and is not [flushable](stylish::io::Write::flush).
/// If you only want to accept Unicode and you don’t need flushing, you should implement this
/// trait; otherwise you should implement [`stylish::io::Write`].
pub trait Write {
    /// Writes a string slice with a particular [`Style`] into this writer, returning whether the write
    /// succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, Write};
    ///
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_str("water is ", Style::default())?;
    ///     output.write_str("blue", Style::default().with(Foreground(Color::Blue)))?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(s, "water is <span style=color:blue>blue</span>");
    /// # Ok::<(), std::fmt::Error>(())
    /// ```
    fn write_str(&mut self, s: &str, style: Style) -> Result;

    /// Writes a [`char`] with a particular [`Style`] into this writer, returning whether the write
    /// succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte. This method can only succeed if the
    /// entire byte sequence was successfully written, and this method will not return until all
    /// data has been written or an error occurs.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, Write};
    ///
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_char('⚠', Style::default().with(Foreground(Color::Yellow)))?;
    ///     output.write_char(' ', Style::default())?;
    ///     output.write_char('⛔', Style::default().with(Foreground(Color::Red)))?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(
    ///     s,
    ///     "<span style=color:yellow>⚠</span> <span style=color:red>⛔</span>"
    /// );
    /// # Ok::<(), std::fmt::Error>(())
    /// ```
    fn write_char(&mut self, c: char, style: Style) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]), style)
    }

    /// Glue for usage of the [`stylish::write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through the
    /// [`stylish::write!`] macro itself.
    ///
    /// ```rust
    /// use stylish::{Color, Foreground, Style, Write};
    ///
    /// let mut s = String::new();
    /// {
    ///     let mut output = stylish::html(&mut s);
    ///     output.write_fmt(stylish::format_args!("{:(fg=red)}", '☎'))?;
    ///     output.finish()?;
    /// }
    ///
    /// assert_eq!(s, "<span style=color:red>☎</span>");
    /// # Ok::<(), std::fmt::Error>(())
    /// ```
    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        Formatter::new(&mut self).write_fmt(args)
    }
}

impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        (&mut **self).write_str(s, style)
    }

    fn write_char(&mut self, c: char, style: Style) -> Result {
        (&mut **self).write_char(c, style)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        (&mut **self).write_fmt(args)
    }
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::__export::stylish_macros::format_args_nl!(crate=$crate, $($arg)*))
    };
}
