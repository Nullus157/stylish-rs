#![no_std]

//! Yet another crate implementing colorized output.
//!
//! There was one primary design goal separating `stylish` from existing crates:
//!
//! <blockquote><span style=font-size:1.3em>
//!
//! Applying styling to data should be decoupled from how that styling is output.
//!
//! </span></blockquote>
//!
//! This came out of two usecases:
//!
//!  1. A library crate that renders a "diagnostic" representation of a data format (think
//!     something JSON-like). This library is being used in both a WASM based web application and a
//!     CLI application; in both cases these applications would be improved by adding some syntax
//!     highlighting to the rendered data, but in one case we want to output HTML while the other
//!     requires ANSI color codes.
//!
//!  2. A (different) CLI application which could use semantic coloring of different data types
//!     embedded in the output messages to make them easier to parse, with an option to turn the
//!     color off. To simplify toggling the color the rendering of the messages shouldn't need to
//!     continuously check whether color is currently on or not.
//!
//! Along with this primary design goal, there was a secondary design goal:
//!
//! <blockquote><span style=font-size:1.1em>
//!
//! Integrate into `core::fmt` as much as possible to leverage existing knowledge.
//!
//! </span></blockquote>
//!
//! We already have a standardized formatting infrastructure in [`core::fmt`]. Developers already
//! know how to work with this, and it is very easy to use. By reusing that existing design and
//! just extending it where needed it should be trivial to get started with `stylish`.
//!
//! # Writing data with attributes
//!
//! There are two primary mechanisms you can use to output data with attached attributes; either
//! applying the attributes as part of the format string, or implementing
//! [`stylish::Display`] to be able to print some type with attributes.
//!
//! ## Applying attributes in format string
//!
//! `stylish`'s macros extend the standard [`fmt` parameters][std::fmt#formatting-parameters] to
//! support setting attributes within `()`. These must come at the end of the parameters just
//! before selecting which trait.
//!
//! ```rust
//! assert_eq!(
//!     stylish::html::format!("Hello {:(fg=red)}", "Ferris"),
//!     "Hello <span style=color:red>Ferris</span>",
//! );
//! ```
//!
//! ### Allowed attributes
//!
//! There are two parameterised attributes, and 3 non-parameterised attributes:
//!
//!   * `fg` specifies a [`Foreground`] style and takes a [`Color`] value in lowercase
//!   * `bg` specifies a [`Background`] style and also takes a [`Color`] value in lowercase
//!   * `bold`, `normal` and `faint` take no parameters and specify an [`Intensity`] style
//!
//! ### Syntax change
//!
//! The specific syntax change is extending `format_spec` like so:
//!
//! ```text
//! format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][attributes][type]
//! attributes := '(' [attribute [',' attribute]* [',']] ')'
//! attribute := key ['=' value]
//! key := identifier
//! value := identifier
//! ```
//!
//! ## Implementing a style for a type
//!
//! [`stylish::Display`] is similar to [`core::fmt::Display`] but with a
//! [`Formatter`] that supports setting style attributes. It can be specified by using the
//! trait-selector `s` in a format string. See the [`Formatter`] docs for more details on how you
//! can programmatically set the styles as you write out your data.
//!
//! ```rust
//! struct Name(&'static str);
//!
//! impl stylish::Display for Name {
//!     fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
//!         let color = match self.0 {
//!             "Ferris" => stylish::Color::Red,
//!             "Gorris" => stylish::Color::Cyan,
//!             _ => stylish::Color::Default,
//!         };
//!         f.with(stylish::Foreground(color)).write_str(self.0)
//!     }
//! }
//!
//! assert_eq!(
//!     stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris")),
//!     "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>",
//! );
//! ```

#[cfg(feature = "std")]
extern crate std;

#[cfg(doc)]
extern crate self as stylish;

#[cfg(all(doc, not(feature = "std")))]
use core::fmt as doc_fmt;
#[cfg(all(doc, feature = "std"))]
use std::fmt as doc_fmt;

pub use stylish_core::{
    format, format_args, write, writeln, Arguments, Background, Color, Display, Error, Foreground,
    Formatter, Intensity, Restyle, Result, Style, StyleDiff, ToStylishString, Write,
};

#[cfg(feature = "alloc")]
pub use stylish_core::String;

#[cfg(feature = "std")]
pub use stylish_core::io;

/// An alias for [`stylish::ansi::Ansi::new`] for more succinct code.
///
/// ```rust
/// let mut writer = stylish::ansi(String::new());
/// stylish::write!(writer, "Hello {:(fg=red)}", "Ferris");
/// assert_eq!(
///     writer.finish()?,
///     "Hello \x1b[31mFerris\x1b[0m",
/// );
/// # Ok::<(), core::fmt::Error>(())
/// ```
pub fn ansi<T: core::fmt::Write>(inner: T) -> ansi::Ansi<T> {
    ansi::Ansi::new(inner)
}

pub fn html<T: core::fmt::Write>(inner: T) -> html::Html<T> {
    html::Html::new(inner)
}

pub fn plain<T>(inner: T) -> plain::Plain<T> {
    plain::Plain::new(inner)
}

#[doc(inline)]
pub use stylish_ansi as ansi;
#[doc(inline)]
pub use stylish_html as html;
#[doc(inline)]
pub use stylish_plain as plain;
