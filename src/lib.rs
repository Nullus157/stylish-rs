#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(variant_size_differences)]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(doc, feature = "alloc"))]
extern crate alloc;

#[cfg(doc)]
extern crate self as stylish;

#[cfg(all(doc, not(feature = "std")))]
use core::fmt as doc_fmt;
#[cfg(all(doc, feature = "std"))]
use std::fmt as doc_fmt;

#[cfg(all(feature = "alloc", feature = "macros"))]
pub use stylish_core::ToStylishString;
#[cfg(feature = "alloc")]
pub use stylish_core::{format, String};
#[cfg(feature = "macros")]
pub use stylish_core::{format_args, write, writeln};
pub use stylish_core::{
    Arguments, Background, Color, Display, Error, Foreground, Formatter, Intensity, Restyle,
    Result, Style, StyleDiff, Write,
};

#[cfg(feature = "std")]
pub mod io {
    //! Traits and associated types for writing attributed data to fallible
    //! IO sinks.

    #[cfg(feature = "ansi")]
    pub use stylish_ansi::io::Ansi;
    pub use stylish_core::io::{Error, ErrorKind, Result, Write};

    #[cfg(feature = "ansi")]
    /// An alias for [`stylish::io::Ansi::new`] for more succinct code.
    ///
    /// ```rust
    /// let mut writer = stylish::io::ansi(Vec::new());
    /// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
    /// assert_eq!(writer.finish()?, b"Hello \x1b[31mFerris\x1b[0m");
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn ansi<T: std::io::Write>(inner: T) -> Ansi<T> {
        Ansi::new(inner)
    }

    #[cfg(feature = "plain")]
    pub use stylish_plain::io::Plain;

    #[cfg(feature = "plain")]
    /// An alias for [`stylish::io::Plain::new`] for more succinct code.
    ///
    /// ```rust
    /// let mut writer = stylish::io::plain(Vec::new());
    /// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
    /// assert_eq!(writer.into_inner(), b"Hello Ferris");
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn plain<T: std::io::Write>(inner: T) -> Plain<T> {
        Plain::new(inner)
    }
}

#[cfg(feature = "ansi")]
/// An alias for [`stylish::Ansi::new`] for more succinct code.
///
/// ```rust
/// let mut writer = stylish::ansi(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
/// assert_eq!(writer.finish()?, "Hello \x1b[31mFerris\x1b[0m");
/// # Ok::<(), core::fmt::Error>(())
/// ```
pub fn ansi<T: core::fmt::Write>(inner: T) -> Ansi<T> {
    Ansi::new(inner)
}

#[cfg(feature = "html")]
/// An alias for [`stylish::Html::new`] for more succinct code.
///
/// ```rust
/// let mut writer = stylish::html(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
/// assert_eq!(
///     writer.finish()?,
///     "Hello <span style=color:red>Ferris</span>",
/// );
/// # Ok::<(), core::fmt::Error>(())
/// ```
pub fn html<T: core::fmt::Write>(inner: T) -> Html<T> {
    Html::new(inner)
}

#[cfg(feature = "plain")]
/// An alias for [`stylish::Plain::new`] for more succinct code.
///
/// ```rust
/// let mut writer = stylish::plain(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris")?;
/// assert_eq!(writer.into_inner(), "Hello Ferris");
/// # Ok::<(), core::fmt::Error>(())
/// ```
pub fn plain<T: core::fmt::Write>(inner: T) -> Plain<T> {
    Plain::new(inner)
}

#[cfg(feature = "ansi")]
pub use stylish_ansi::Ansi;
#[cfg(all(feature = "ansi", feature = "alloc", feature = "macros"))]
pub use stylish_ansi::ToAnsiString;
#[cfg(feature = "ansi")]
pub mod ansi {
    //! Helpers for writing styles as ANSI escape codes.

    #[cfg(feature = "alloc")]
    pub use stylish_ansi::format;
}

#[cfg(feature = "html")]
pub use stylish_html::Html;
#[cfg(all(feature = "html", feature = "alloc", feature = "macros"))]
pub use stylish_html::ToHtmlString;
#[cfg(feature = "html")]
pub mod html {
    //! Helpers for writing styles as HTML elements.

    #[cfg(feature = "alloc")]
    pub use stylish_html::format;
}

#[cfg(feature = "plain")]
pub use stylish_plain::Plain;
#[cfg(all(feature = "plain", feature = "alloc", feature = "macros"))]
pub use stylish_plain::ToPlainString;
#[cfg(feature = "plain")]
pub mod plain {
    //! Helpers for discarding styles.

    #[cfg(feature = "alloc")]
    pub use stylish_plain::format;
}
