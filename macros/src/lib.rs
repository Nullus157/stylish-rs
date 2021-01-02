use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, LitStr, Token,
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
    // With {
    //     restyle: &'a dyn style::Apply,
    //     arguments: Arguments<'a>,
    // }

    let span = format.span();
    let format = format.value();
    let (leftover, format) = format::Format::parse(&format).unwrap();
    let mut args = args.into_iter();
    let pieces: Vec<_> = format
        .pieces
        .into_iter()
        .chain(std::iter::once(format::Piece::Lit(leftover)))
        .map(|piece| match piece {
            format::Piece::Lit(lit) => {
                let lit = LitStr::new(lit, span);
                quote!(stylish::Argument::Lit(#lit))
            }
            format::Piece::Arg(format::FormatArg { variant }) => {
                let arg = args.next().expect("missing argument");
                match variant {
                    format::Variant::Display => {
                        quote!(stylish::Argument::Display(&#arg))
                    }
                    format::Variant::Debug(alternate) => {
                        quote!(stylish::Argument::Debug(#alternate, &#arg))
                    }
                }
            }
        })
        .collect();
    quote! {
        stylish::Arguments {
            pieces: &[
                #(#pieces),*
            ],
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
    quote!({ use stylish::Write; #target.write_fmt(&#args) })
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WriteInput { target, args, .. } = parse_macro_input!(input as WriteInput);
    let args = format_args_impl(args);
    quote!({ use stylish::Write; #target.write_fmt(&#args) })
        .into_token_stream()
        .into()
}
