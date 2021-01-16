#![no_std]

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
//! <span class="macro">assert_eq</span><span class="macro">!</span>(<span class="ident">formatted</span>, <span class="string">r#"<span style="color: inherit; background-color: inherit; font-weight: inherit">Hello </span><span style="color: red; background-color: inherit; font-weight: inherit">Ferris</span>"#</span>);
//! </pre>
//! </div>
//! </summary>
//!
//! ```rust
//! let formatted = stylish::html::format!("Hello {:(fg=red)}", "Ferris");
//! assert_eq!(formatted, r#"<span style="color: inherit; background-color: inherit; font-weight: inherit">Hello </span><span style="color: red; background-color: inherit; font-weight: inherit">Ferris</span>"#);
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
//! attributes := '(' [attribute] [',' attribute]* [','] ')'
//! attribute := key ['=' value]
//! key := identifier
//! value := identifier
//! ```
//!

//! ## Implementing a style for a type
//!
//! [`stylish::Display`] is similar to [`std::fmt::Display`][`doc_fmt::Display`] but with a
//! [`Formatter`] that supports setting style attributes. It can be specified by using the
//! trait-selector `s` in a format string.
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
//!             <span class="string">"Gorris"</span> <span class="op">=</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="ident">Color</span>::<span class="ident">Blue</span>,
//!             <span class="kw">_</span> <span class="op">=</span><span class="op">&gt;</span> <span class="ident">stylish</span>::<span class="ident">Color</span>::<span class="ident">Default</span>,
//!         };
//!         <span class="ident">f</span>.<span class="ident">with</span>(<span class="ident">stylish</span>::<span class="ident">Foreground</span>(<span class="ident">color</span>)).<span class="ident">write_str</span>(<span class="self">self</span>.<span class="number">0</span>)
//!     }
//! }
//! <span></span>
//! <span class="kw">let</span> <span class="ident">formatted</span> <span class="op">=</span> <span class="ident">stylish</span>::<span class="ident">html</span>::<span class="macro">format</span><span class="macro">!</span>(<span class="string">"Hello {:s} and {:s}"</span>, <span class="ident">Name</span>(<span class="string">"Ferris"</span>), <span class="ident">Name</span>(<span class="string">"Gorris"</span>));
//! <span class="macro">assert_eq</span><span class="macro">!</span>(<span class="ident">formatted</span>, <span class="string">r#"<span style="color: inherit; background-color: inherit; font-weight: inherit">Hello </span><span style="color: red; background-color: inherit; font-weight: inherit">Ferris</span><span style="color: inherit; background-color: inherit; font-weight: inherit"> and </span><span style="color: blue; background-color: inherit; font-weight: inherit">Gorris</span>"#</span>);
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
//!             "Gorris" => stylish::Color::Blue,
//!             _ => stylish::Color::Default,
//!         };
//!         f.with(stylish::Foreground(color)).write_str(self.0)
//!     }
//! }
//!
//! let formatted = stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris"));
//! assert_eq!(formatted, r#"<span style="color: inherit; background-color: inherit; font-weight: inherit">Hello </span><span style="color: red; background-color: inherit; font-weight: inherit">Ferris</span><span style="color: inherit; background-color: inherit; font-weight: inherit"> and </span><span style="color: blue; background-color: inherit; font-weight: inherit">Gorris</span>"#);
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
