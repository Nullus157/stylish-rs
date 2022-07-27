use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use stylish_style::{Intensity, Color};

use crate::format::{
    Align, Count, DebugHex, FormatTrait, FormatterArgs, Restyle, Restyles, Sign,
};

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

impl<'a> ToTokens for Scoped<'a, Restyle> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            Restyle::Foreground(color) => {
                let color = self.scope(color);
                quote!(#export::Foreground(#color))
            }
            Restyle::Background(color) => {
                let color = self.scope(color);
                quote!(#export::Background(#color))
            }
            Restyle::Intensity(intensity) => {
                let intensity = self.scope(intensity);
                quote!(#intensity)
            }
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, Restyles> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Restyles { restyles } = self.as_ref();
        let mut style = quote!(());
        for restyle in restyles {
            let restyle = self.scope(restyle);
            style = quote!((#restyle, #style));
        }
        (quote! { &#style }).to_tokens(tokens)
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
            Count::Parameter(_) => todo!(),
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
            #export::FormatterArgs {
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
            FormatTrait::Stylish => quote!(Stylish),
        }
        .to_tokens(tokens)
    }
}

impl<'a> ToTokens for Scoped<'a, (FormatTrait, TokenStream)> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            (FormatTrait::Stylish, arg) => {
                quote!(#export::FormatTrait::Stylish(#arg))
            }
            (format_trait, arg) => {
                quote!(#export::FormatTrait::#format_trait(match #arg { __stylish_arg => #export::StdFmt::new(move |f| #export::fmt::#format_trait::fmt(__stylish_arg, f)) }))
            }
        }
        .to_tokens(tokens)
    }
}
