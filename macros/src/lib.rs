use self::{
    format::{Format, FormatArg, FormatArgRef, FormatSpec, Piece},
    to_tokens::Scoped,
};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, ExprAssign, ExprPath, Ident, Index, LitStr, Path, PathArguments,
    Token,
};

mod format;
mod to_tokens;

struct ArgsInput {
    krate: Option<Path>,
    format: LitStr,
    positional_args: Vec<Expr>,
    named_args: Vec<(Ident, Expr)>,
}

impl Parse for ArgsInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let krate = if input.peek(Token![crate]) {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;
            let res = input.parse()?;
            input.parse::<Token![,]>()?;
            Some(res)
        } else {
            None
        };
        let format = input.parse()?;
        let mut positional_args = Vec::new();
        let mut named_args = Vec::new();
        let mut onto_named = false;
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let expr = input.parse::<Expr>()?;
            match expr {
                Expr::Assign(ExprAssign { left, right, .. }) if matches!(&*left, Expr::Path(ExprPath { path, .. }) if path.segments.len() == 1 && matches!(path.segments[0].arguments, PathArguments::None)) =>
                {
                    let ident = if let Expr::Path(ExprPath { mut path, .. }) = *left {
                        path.segments.pop().unwrap().into_value().ident
                    } else {
                        panic!()
                    };
                    named_args.push((ident, *right));
                    onto_named = true;
                }
                expr => {
                    if onto_named {
                        panic!("positional arg after named")
                    }
                    positional_args.push(expr);
                }
            }
        }
        Ok(Self {
            krate,
            format,
            positional_args,
            named_args,
        })
    }
}

fn format_args_impl(
    ArgsInput {
        krate,
        format,
        positional_args,
        named_args,
    }: ArgsInput,
) -> impl ToTokens {
    let krate = krate.expect("base crate not specified (are you using stylish-macros directly instead of through stylish-core?)");
    let export: syn::Path = syn::parse_quote!(#krate::__export);

    let span = format.span();
    let format_string = &format;
    let format = format.value();
    let (leftover, format) = Format::parse(&format).unwrap();
    assert!(leftover.is_empty());
    let num_positional_args = positional_args.len();
    let positional_args_ident = Ident::new("__stylish_positional_args", Span::mixed_site());
    let named_args_ident = Ident::new("__stylish_named_args", Span::mixed_site());
    let positional_args = positional_args.into_iter();
    let positional_args = quote! {
        (#(&#positional_args,)*)
    };
    let (named_args_names, named_args_values): (Vec<_>, Vec<_>) = named_args.into_iter().unzip();
    let named_args_names: HashMap<String, usize> = named_args_names
        .into_iter()
        .map(|name| name.to_string())
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let named_args_values = named_args_values.into_iter();
    let named_args = quote! {
        (#(&#named_args_values,)*)
    };
    let implicit_named_args_ident = Ident::new("__stylish_implicit_named_args", Span::mixed_site());
    let mut implicit_named_args_values = Vec::new();
    let mut next_arg_iter = (0..num_positional_args).map(Index::from);
    let pieces: Vec<_> = format
        .pieces
        .into_iter()
        .map(|piece| match piece {
            Piece::Lit(lit) => {
                let lit = LitStr::new(&lit.replace("{{", "{"), span);
                quote!(#export::Argument::Lit(#lit))
            }
            Piece::Arg(FormatArg {
                arg,
                format_spec:
                    FormatSpec {
                        formatter_args,
                        format_trait,
                    },
            }) => {
                let formatter_args = Scoped::new(&export, &formatter_args);
                let format_trait = Scoped::new(&export, &format_trait);
                let arg = match arg {
                    None => {
                        let index = next_arg_iter.next().expect("missing argument");
                        quote!(#positional_args_ident.#index)
                    }
                    Some(FormatArgRef::Positional(i)) => {
                        let index = Index::from(i);
                        quote!(#positional_args_ident.#index)
                    }
                    Some(FormatArgRef::Named(name)) => {
                        if let Some(&i) = named_args_names.get(name) {
                            let index = Index::from(i);
                            quote!(#named_args_ident.#index)
                        } else {
                            let i = implicit_named_args_values.len();
                            implicit_named_args_values.push(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: Ident::new(
                                    name,
                                    Span::call_site().resolved_at(format_string.span()),
                                )
                                .into(),
                            });
                            let index = Index::from(i);
                            quote!(#implicit_named_args_ident.#index)
                        }
                    }
                };
                quote!(#export::arg(#formatter_args, move |f| #format_trait::fmt(#arg, f)))
            }
        })
        .collect();
    let implicit_named_args = quote! {
        (#(&#implicit_named_args_values,)*)
    };
    quote! {
        #export::Arguments {
            pieces: &match (#positional_args, #named_args, #implicit_named_args) {
                (#positional_args_ident, #named_args_ident, #implicit_named_args_ident) => [
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
pub fn format_args_nl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ArgsInput);
    let format = LitStr::new(&(input.format.value() + "\n"), input.format.span());
    format_args_impl(ArgsInput { format, ..input })
        .into_token_stream()
        .into()
}
