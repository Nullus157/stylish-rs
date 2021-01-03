use self::format::{Format, FormatArgRef, Piece, Variant};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Index, LitStr, Token,
};

mod format;

struct ArgsInput {
    format: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for ArgsInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let format = input.parse()?;
        let args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse_terminated(Expr::parse)?
        } else {
            Punctuated::new()
        };
        Ok(Self { format, args })
    }
}

struct WriteInput {
    target: Expr,
    args: ArgsInput,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let target = input.parse()?;
        input.parse::<Token![,]>()?;
        let args = input.parse()?;
        Ok(Self { target, args })
    }
}

struct WriteLnInput {
    target: Expr,
    args: Option<ArgsInput>,
}

impl Parse for WriteLnInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let target = input.parse()?;
        let args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { target, args })
    }
}

fn format_args_impl(ArgsInput { format, args }: ArgsInput) -> impl ToTokens {
    let span = format.span();
    let format = format.value();
    let (leftover, format) = Format::parse(&format).unwrap();
    assert!(leftover.is_empty());
    let num_args = args.len();
    let positional_args_ident = Ident::new("__stylish_positional_args", Span::call_site());
    let args = args.into_iter();
    let positional_args = quote! {
        (#(&#args,)*)
    };
    let mut next_arg_iter = (0..num_args).map(Index::from);
    let pieces: Vec<_> = format
        .pieces
        .into_iter()
        .map(|piece| match piece {
            Piece::Lit(lit) => {
                let lit = LitStr::new(&lit.replace("{{", "{"), span);
                quote!(stylish::Argument::Lit(#lit))
            }
            Piece::Arg(format::FormatArg { variant, arg, args }) => {
                let arg = match arg {
                    FormatArgRef::Next => {
                        let index = next_arg_iter.next().expect("missing argument");
                        quote!(#positional_args_ident.#index)
                    }
                    FormatArgRef::Positional(i) => {
                        let index = Index::from(i);
                        quote!(#positional_args_ident.#index)
                    }
                    FormatArgRef::Named(_) => unimplemented!(),
                };
                match variant {
                    Variant::Display => {
                        quote!(stylish::Argument::Display(#args, #arg))
                    }
                    Variant::Debug => {
                        quote!(stylish::Argument::Debug(#args, #arg))
                    }
                }
            }
        })
        .collect();
    quote! {
        stylish::Arguments {
            pieces: &match #positional_args {
                #positional_args_ident => [
                    #(#pieces),*
                ],
            }
        }
    }
}

#[proc_macro]
pub fn format_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    format_args_impl(parse_macro_input!(input as ArgsInput))
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn format_plain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = format_args_impl(parse_macro_input!(input as ArgsInput));
    quote!({
        let mut string = stylish::plain::String::new();
        stylish::fmt::Write::write_fmt(&mut string, &#args).unwrap();
        string.into_inner()
    })
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn format_ansi(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = format_args_impl(parse_macro_input!(input as ArgsInput));
    quote!({
        let mut string = stylish::ansi::String::new();
        stylish::fmt::Write::write_fmt(&mut string, &#args).unwrap();
        string.into_inner()
    })
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn format_html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = format_args_impl(parse_macro_input!(input as ArgsInput));
    quote!({
        let mut string = stylish::html::String::new();
        stylish::fmt::Write::write_fmt(&mut string, &#args).unwrap();
        string.into_inner()
    })
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn writeln(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WriteLnInput { target, args } = parse_macro_input!(input as WriteLnInput);
    let args = if let Some(args) = args {
        format_args_impl(ArgsInput {
            format: LitStr::new(&(args.format.value() + "\n"), args.format.span()),
            args: args.args,
        })
    } else {
        format_args_impl(ArgsInput {
            format: LitStr::new("\n", Span::call_site()),
            args: Punctuated::new(),
        })
    };
    quote!({ #target.write_fmt(&#args) })
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WriteInput { target, args, .. } = parse_macro_input!(input as WriteInput);
    let args = format_args_impl(args);
    quote!({ #target.write_fmt(&#args) })
        .into_token_stream()
        .into()
}
