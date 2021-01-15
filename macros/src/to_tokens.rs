use crate::format::{Align, Count, DebugHex, FormatTrait, FormatterArgs, Restyle, Sign};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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

impl<'a, 'b: 'a> ToTokens for Scoped<'a, Restyle<'b>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        let Restyle { color } = self.as_ref();
        let color = match color {
            Some("blue") => quote!(&#export::Color::Blue),
            Some(color) => panic!("Unknown color {}", color),
            None => quote!(&()),
        };
        (quote! { #color }).to_tokens(tokens)
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

impl<'a> ToTokens for Scoped<'a, FormatTrait> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let export = &self.export;
        match self.as_ref() {
            FormatTrait::Display => {
                quote!(#export::Display)
            }
            FormatTrait::Debug => {
                quote!(#export::Debug)
            }
            FormatTrait::Octal => {
                quote!(#export::Octal)
            }
            FormatTrait::LowerHex => {
                quote!(#export::LowerHex)
            }
            FormatTrait::UpperHex => {
                quote!(#export::UpperHex)
            }
            FormatTrait::Pointer => {
                quote!(#export::Pointer)
            }
            FormatTrait::Binary => {
                quote!(#export::Binary)
            }
            FormatTrait::LowerExp => {
                quote!(#export::LowerExp)
            }
            FormatTrait::UpperExp => {
                quote!(#export::UpperExp)
            }
        }
        .to_tokens(tokens)
    }
}
