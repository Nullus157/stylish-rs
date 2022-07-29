Yet another crate implementing colorized text.

There was one primary design goal separating [`stylish`][] from existing crates:

<blockquote><span style=font-size:1.3em>

Applying styling to data should be decoupled from how that styling is output.

</span></blockquote>

This came out of two usecases:

 1. A library crate that renders a "diagnostic" representation of a data format
    (think something JSON-like). This library is being used in both a WASM based
    web application and a CLI application; in both cases these applications
    would be improved by adding some syntax highlighting to the rendered data,
    but in one case we want to output HTML while the other requires ANSI color
    codes.

 2. A (different) CLI application which could use semantic coloring of different
    data types embedded in the output messages to make them easier to parse,
    with an option to turn the color off. To simplify toggling the color the
    rendering of the messages shouldn't need to continuously check whether color
    is currently on or not.

Along with this primary design goal, there was a secondary design goal:

<blockquote><span style=font-size:1.1em>

Integrate into [`std::fmt`][] as much as possible to leverage existing
knowledge.

</span></blockquote>

We already have a standardized formatting infrastructure in [`std::fmt`][].
Developers already know how to work with this, and it is very easy to use.  By
reusing that existing design and just extending it where needed it should be
trivial to get started with [`stylish`][].

[`std::fmt`]: std::fmt
[`stylish`]: ::stylish

# Writing data with attributes

There are two primary mechanisms you can use to output data with attached
attributes; either applying the attributes as part of the format string, or
implementing [`stylish::Display`][] to be able to print some type with attributes.

[`stylish::Display`]: stylish::Display

## Applying attributes in format string

[`stylish`][]'s macros extend the standard [`fmt` parameters][] to support
setting attributes within `()`. These must come at the end of the parameters
just before selecting which trait.

```rust
assert_eq!(
    stylish::html::format!("Hello {:(fg=red)}", "Ferris"),
    "Hello <span style=color:red>Ferris</span>",
);
```

[`fmt` parameters]: std::fmt#formatting-parameters

### Allowed attributes

There are two parameterised attributes, and 3 non-parameterised attributes:

 * `fg` specifies a [`Foreground`][] style and takes a [`Color`][] value in
   lowercase

 * `bg` specifies a [`Background`][] style and also takes a [`Color`][] value
   in lowercase

 * `bold`, `normal` and `faint` take no parameters and specify an
   [`Intensity`][] style

[`Background`]: stylish::Background
[`Color`]: stylish::Color
[`Foreground`]: stylish::Foreground
[`Intensity`]: stylish::Intensity

### Syntax change

The specific syntax change is extending [`format_spec`][] like so:

```text
format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][attributes]type
attributes := '(' [attribute [',' attribute]* [',']] ')'
attribute := key ['=' value]
key := identifier
value := identifier
```

[`format_spec`]: std::fmt#syntax

## Implementing a style for a type

[`stylish::Display`][] is similar to [`std::fmt::Display`][] but with a
[`Formatter`][] that supports setting style attributes. It can be specified by
using the trait-selector `s` in a format string. See the [`Formatter`][] docs for
more details on how you can programmatically set the styles as you write out
your data.

```rust
struct Name(&'static str);

impl stylish::Display for Name {
    fn fmt(&self, f: &mut stylish::Formatter<'_>) -> stylish::Result {
        let color = match self.0 {
            "Ferris" => stylish::Color::Red,
            "Gorris" => stylish::Color::Cyan,
            _ => stylish::Color::Default,
        };
        f.with(stylish::Foreground(color)).write_str(self.0)
    }
}

assert_eq!(
    stylish::html::format!("Hello {:s} and {:s}", Name("Ferris"), Name("Gorris")),
    "Hello <span style=color:red>Ferris</span> and <span style=color:cyan>Gorris</span>",
);
```

[`stylish::Display`]: stylish::Display
[`std::fmt::Display`]: std::fmt::Display
[`Formatter`]: stylish::Formatter

# Features

| Feature  | Activation         | Effect
|----------|--------------------|--------
| `std`    | **on**-by-default  | Enables the [`io`][] module (and `io` helpers in other modules)
| `alloc`  | implied by `std`   | Enables [`String`][] and a variety of items that use it
| `macros` | **on**-by-default  | Enables macros throughout the other enabled modules
| `ansi`   | *off*-by-default   | Enables the [`ansi`][] module and items that use it
| `html`   | *off*-by-default   | Enables the [`html`][] module and items that use it
| `plain`  | *off*-by-default   | Enables the [`plain`][] module and items that use it

[`io`]: stylish::io
[`String`]: stylish::String
[`ansi`]: mod@stylish::ansi
[`html`]: mod@stylish::html
[`plain`]: mod@stylish::plain
