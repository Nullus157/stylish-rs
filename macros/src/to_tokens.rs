// TODO: Use this to ensure we update for the `StyleDiff` and associated types
// https://github.com/rust-lang/rust/issues/89554
// #![warn(non_exhaustive_omitted_patterns)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use stylish_style::{Color, Foreground, Intensity, StyleDiff};

use crate::format::{Align, Count, DebugHex, FormatTrait, FormatterArgs, Sign};

fn quote_opt<'a, T: 'a>(opt: Scoped<'a, Option<T>>) -> TokenStream
where
    Scoped<'a, T>: ToTokens,
{
    let export = &opt.export;
    match opt.as_ref() {
        Some(value) => {
            let value = opt.scope(value);
            quote!(#export::Option::Some(#value))
        }
        None => quote!(#export::Option::None),
    }
}

pub struct Scoped<'a, T> {
    export: &'a syn::Path,
    inner: &'a T,
}

impl<'a, T> Scoped<'a, T> {
    pub fn new(export: &'a syn::Path, inner: &'a T) -> Self {
        Self { export, inner }
    }

    fn scope<'b, U>(&self, inner: &'b U) -> Scoped<'b, U>
    where
        'a: 'b,
    {
        Scoped {
            inner,
            export: self.export,
        }
    }

    fn as_ref(&self) -> &'a T {
        self.inner
    }
}

impl<'a> ToTokens for Scoped<'a, Color> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            Color::Black => quote!(#export::Color::Black),
            Color::Red => quote!(#export::Color::Red),
            Color::Green => quote!(#export::Color::Green),
            Color::Yellow => quote!(#export::Color::Yellow),
            Color::Blue => quote!(#export::Color::Blue),
            Color::Magenta => quote!(#export::Color::Magenta),
            Color::Cyan => quote!(#export::Color::Cyan),
            Color::White => quote!(#export::Color::White),
            Color::Default => quote!(#export::Color::Default),
            color => unreachable!("unknown color {color:?}"),
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, Intensity> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            Intensity::Normal => quote!(#export::Intensity::Normal),
            Intensity::Bold => quote!(#export::Intensity::Bold),
            Intensity::Faint => quote!(#export::Intensity::Faint),
            intensity => unreachable!("unknown intensity {intensity:?}"),
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, Foreground> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        let Foreground(color) = self.as_ref();
        let color = self.scope(color);
        quote!(#export::Foreground(#color)).to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, StyleDiff> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        let StyleDiff {
            foreground,
            background,
            intensity,
            ..
        } = self.as_ref();
        let mut inner = TokenStream::new();
        quote!(let mut diff = #export::StyleDiff::default();).to_tokens(&mut inner);
        if let Some(foreground) = foreground {
            let foreground = self.scope(foreground);
            quote!(diff.foreground = Some(#foreground);).to_tokens(&mut inner);
        }
        if let Some(background) = background {
            let background = self.scope(background);
            quote!(diff.background = Some(#background);).to_tokens(&mut inner);
        }
        if let Some(intensity) = intensity {
            let intensity = self.scope(intensity);
            quote!(diff.intensity = Some(#intensity);).to_tokens(&mut inner);
        }
        quote!(diff).to_tokens(&mut inner);
        quote!({ #inner }).to_tokens(tokens);
    }
}

impl<'a> ToTokens for Scoped<'a, Align> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            Align::Left => quote!(#export::Align::Left),
            Align::Center => quote!(#export::Align::Center),
            Align::Right => quote!(#export::Align::Right),
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, Sign> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            Sign::Plus => quote!(#export::Sign::Plus),
            Sign::Minus => quote!(#export::Sign::Minus),
        }
        .to_tokens(tokens)
    }
}

impl<'a, 'b: 'a> ToTokens for Scoped<'a, Count<'b>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.as_ref() {
            Count::Parameter(_value) => todo!("parameter reference counts are not yet supported"),
            Count::Integer(value) => quote!(&#value).to_tokens(tokens),
        }
    }
}

impl<'a> ToTokens for Scoped<'a, DebugHex> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            DebugHex::Lower => quote!(#export::DebugHex::Lower),
            DebugHex::Upper => quote!(#export::DebugHex::Upper),
        }
        .to_tokens(tokens)
    }
}

impl<'a, 'b: 'a> ToTokens for Scoped<'a, FormatterArgs<'b>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        let FormatterArgs {
            align,
            sign,
            alternate,
            zero,
            width,
            precision,
            debug_hex,
        } = self.as_ref();
        let align = quote_opt(self.scope(align));
        let sign = quote_opt(self.scope(sign));
        let width = quote_opt(self.scope(width));
        let precision = quote_opt(self.scope(precision));
        let debug_hex = quote_opt(self.scope(debug_hex));
        (quote! {
            &#export::FormatterArgs {
                align: #align,
                sign: #sign,
                alternate: #alternate,
                zero: #zero,
                width: #width,
                precision: #precision,
                debug_hex: #debug_hex,
            }
        })
        .to_tokens(tokens)
    }
}

impl ToTokens for FormatTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FormatTrait::Display => quote!(Display),
            FormatTrait::Debug => quote!(Debug),
            FormatTrait::Octal => quote!(Octal),
            FormatTrait::LowerHex => quote!(LowerHex),
            FormatTrait::UpperHex => quote!(UpperHex),
            FormatTrait::Pointer => quote!(Pointer),
            FormatTrait::Binary => quote!(Binary),
            FormatTrait::LowerExp => quote!(LowerExp),
            FormatTrait::UpperExp => quote!(UpperExp),
            FormatTrait::Stylish => unreachable!(),
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, (FormatTrait, TokenStream)> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            (FormatTrait::Stylish, arg) => arg.to_tokens(tokens),
            (format_trait, arg) => {
                let inner = quote! {
                    #export::StdFmt {
                        f: &match #arg {
                            __stylish_arg => {
                                #[inline]
                                move |__stylish_formatter: &mut #export::fmt::Formatter<'_>| -> #export::fmt::Result {
                                    #export::fmt::#format_trait::fmt(__stylish_arg, __stylish_formatter)
                                }
                            }
                        }
                    }
                };
                if let FormatTrait::Debug = format_trait {
                    quote! { #export::StdFmtDebug(#inner) }
                } else {
                    quote! { #export::StdFmtOther(#inner) }
                }
                .to_tokens(tokens)
            }
        }
    }
}
