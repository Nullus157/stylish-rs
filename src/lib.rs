#![no_std]

//! Yet another crate implementing colorized output.
//!
//! There was one primary design goal separating `stylish` from existing crates:
//!
//! <blockquote><span style=font-size:1.3em>
//! Applying styling to data should be decoupled from how that styling is output.
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
//! Integrate into <code>core::fmt</code> as much as possible to leverage existing knowledge.
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
//! `stylish`'s macros extend the standard [`fmt` parameters][doc_fmt#formatting-parameters] to
//! support setting attributes within `()`. These must come at the end of the parameters just
//! before selecting which trait.
//!

//! <details><summary>
//! <div class="example-wrap">
//! <pre class="rust rust-example-rendered">
//! <span class="kw">let</span> <span class="ident">formatted</span> <span class="op">=</span> <span class="ident">stylish</span>::<span class="ident">html</span>::<span class="macro">format</span><span class="macro">!</span>(<span class="string">"Hello {:(fg=red)}"</span>, <span class="string">"Ferris"</span>);
//! <span class="macro">assert_eq</span><span class="macro">!</span>(<span class="ident">formatted</span>, <span class="string">"Hello <span style=color:red>Ferris</span>"</span>);
//! </pre>
//! </div>
//! </summary>
//!
//! ```rust
//! let formatted = stylish::html::format!("Hello {:(fg=red)}", "Ferris");
//! assert_eq!(formatted, "Hello <span style=color:red>Ferris</span>");
//! ```
//! </details>
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
//! [`stylish::Display`] is similar to [`std::fmt::Display`][`doc_fmt::Display`] but with a
//! [`Formatter`] that supports setting style attributes. It can be specified by using the
//! trait-selector `s` in a format string. See the [`Formatter`] docs for more details on how you
//! can programmatically set the styles as you write out your data.
//!

//! <details><summary>
//! <div class="example-wrap">
//! <pre class="rust rust-example-rendered">
//! <span class="kw">struct</span> <span class="ident">Name</span>(<span class="kw-2">&amp;</span><span class="lifetime">'static</span> <span class="ident">str</span>);
//! <span></span>
//! <span class="kw">impl</span> <span class="ident">stylish</span>::<span class="ident">Display</span> <span class="kw">for</span> <span class="ident">Name</span> {
//!     <span class="kw">fn</span> <span class="ident">fmt</span>(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="ident">f</span>: <span class="kw-2">&amp;</span><span class="kw-2">mut</span> <span class="ident">stylish</span>::<span class="ident">Formatter</span><span class="op">&lt;</span><span class="lifetime">'_</span><span class="op">&gt;</span>) <span class="op">-</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="prelude-ty">Result</span> {
//!         <span class="kw">let</span> <span class="ident">color</span> <span class="op">=</span> <span class="kw">match</span> <span class="self">self</span>.<span class="number">0</span> {
//!             <span class="string">"Ferris"</span> <span class="op">=</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="ident">Color</span>::<span class="ident">Red</span>,
//!             <span class="string">"Gorris"</span> <span class="op">=</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="ident">Color</span>::<span class="ident">Cyan</span>,
//!             <span class="kw">_</span> <span class="op">=</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="ident">Color</span>::<span class="ident">Default</span>,
//!         };
//!         <span class="ident">f</span>.<span class="ident">with</span>(<span class="ident">stylish</span>::<span class="ident">Foreground</span>(<span class="ident">color</span>)).<span class="ident">write_str</span>(<span class="self">self</span>.<span class="number">0</span>)
//!     }
//! }
//! <span></span>
//! <span class="kw">let</span> <span class="ident">formatted</span> <span class="op">=</span> <span class="ident">stylish</span>::<span class="ident">html</span>::<span class="macro">format</span><span class="macro">!</span>(<span class="string">"Hello {:s} and {:s}"</span>, <span class="ident">Name</span>(<span class="string">"Ferris"</span>), <span class="ident">Name</span>(<span class="string">"Gorris"</span>));
//! <span class="macro">assert_eq</span><span class="macro">!</span>(<span class="ident">formatted</span>, <span class="string">"Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>"</span>);
//! </pre>
//! </div>
//! </summary>
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
//! let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
//! assert_eq!(formatted, "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>");
//! ```
//! </details>
//!

#[cfg(feature = "std")]
extern crate std;

#[cfg(doc)]
extern crate self as stylish;

#[cfg(all(doc, not(feature = "std")))]
use core::fmt as doc_fmt;

#[cfg(all(doc, feature = "std"))]
use std::fmt as doc_fmt;

pub use stylish_core::{
    format_args, write, writeln, Arguments, Background, Color, Display, Error, Foreground,
    Formatter, Intensity, Result, Style, Write,
};

#[cfg(feature = "std")]
pub use stylish_core::io;

pub fn ansi<T: core::fmt::Write>(inner: T) -> ansi::Ansi<T> {
    ansi::Ansi::new(inner)
}

pub fn html<T: core::fmt::Write>(inner: T) -> html::Html<T> {
    html::Html::new(inner)
}

pub fn plain<T: core::fmt::Write>(inner: T) -> plain::Plain<T> {
    plain::Plain::new(inner)
}

#[doc(inline)]
pub use stylish_ansi as ansi;
#[doc(inline)]
pub use stylish_html as html;
#[doc(inline)]
pub use stylish_plain as plain;
