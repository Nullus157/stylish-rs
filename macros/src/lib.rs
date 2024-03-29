//! Internal implementation details of [`stylish-core`](https://docs.rs/stylish-core).
//!
//! Do not depend on this crate directly.

#![allow(uncommon_codepoints)]
#![cfg_attr(stylish_proc_macro_expand, feature(proc_macro_expand))]

use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{ParseStream, Result},
    parse_macro_input, Expr, ExprAssign, ExprPath, Ident, Index, LitStr, Path, PathArguments,
    Token,
};

use self::{
    format::{Format, FormatArg, FormatArgRef, FormatSpec, Parse as _, Piece},
    to_tokens::Scoped,
};

mod format;
mod to_tokens;

struct ArgsInput {
    krate: Option<Path>,
    format: LitStr,
    positional_args: Vec<Expr>,
    named_args: Vec<(Ident, Expr)>,
}

impl syn::parse::Parse for ArgsInput {
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
        #[cfg(not(stylish_proc_macro_expand))]
        let format = input.parse()?;
        #[cfg(stylish_proc_macro_expand)]
        let format = {
            use syn::spanned::Spanned;
            let expr = input.parse::<Expr>()?;
            let span = expr.span();
            let tokens = proc_macro::TokenStream::from(expr.into_token_stream());
            let expanded = tokens.expand_expr().map_err(|e| {
                syn::parse::Error::new(span, format!("failed to expand format string: {e}"))
            })?;
            #[cfg(stylish_proc_macro_expand_debug)]
            if expanded.to_string() != tokens.to_string() {
                eprintln!("{tokens} => {expanded}");
            }
            syn::parse(expanded)?
        };
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
    let export: syn::Path = syn::parse_quote!(#krate::𓀄);

    let span = format.span();
    let format_string = &format;
    let format = format.value();
    let (leftover, format) = Format::parse(&format).unwrap();
    assert!(leftover.is_empty());
    let num_positional_args = positional_args.len();
    let positional_args = positional_args.into_iter();
    let positional_args = quote! { (#(&#positional_args,)*) };
    let (named_args_names, named_args_values): (Vec<_>, Vec<_>) = named_args.into_iter().unzip();
    let named_args_names: HashMap<String, usize> = named_args_names
        .into_iter()
        .map(|name| name.to_string())
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let named_args_values = named_args_values.into_iter();
    let named_args_values = quote! {
        (#(&#named_args_values,)*)
    };
    let mut implicit_named_args_values = Vec::new();
    let mut used_positional_args = HashSet::new();
    let mut used_named_args = HashSet::new();
    let mut next_arg_iter = 0..num_positional_args;
    let statements: Vec<_> = format
        .pieces
        .into_iter()
        .map(|piece| match piece {
            Piece::Lit(lit) => {
                let lit = LitStr::new(&lit.replace("{{", "{"), span);
                quote!(#export::Formatter::write_str(__stylish_formatter, #lit)?)
            }
            Piece::Arg(FormatArg {
                arg,
                format_spec:
                    FormatSpec {
                        formatter_args,
                        style,
                        format_trait,
                    },
            }) => {
                let formatter_args = Scoped::new(&export, &formatter_args);
                let style = Scoped::new(&export, &style);
                let arg = match arg {
                    None => {
                        let i = next_arg_iter.next().expect("missing argument");
                        let index = Index::from(i);
                        used_positional_args.insert(i);
                        quote!(__stylish_positional_args.#index)
                    }
                    Some(FormatArgRef::Positional(i)) => {
                        let index = Index::from(i);
                        if i >= num_positional_args {
                            panic!("missing positional argument {i}");
                        }
                        used_positional_args.insert(i);
                        quote!(__stylish_positional_args.#index)
                    }
                    Some(FormatArgRef::Named(name)) => {
                        if let Some(&i) = named_args_names.get(name) {
                            let index = Index::from(i);
                            used_named_args.insert(name.to_owned());
                            quote!(__stylish_named_args.#index)
                        } else {
                            let i = implicit_named_args_values.len();
                            implicit_named_args_values.push(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: Ident::new_raw(
                                    name,
                                    Span::call_site().resolved_at(format_string.span()),
                                )
                                .into(),
                            });
                            let index = Index::from(i);
                            quote!(__stylish_implicit_named_args.#index)
                        }
                    }
                };
                let arg = (format_trait, arg);
                let arg = Scoped::new(&export, &arg);
                quote! {
                    #export::Display::fmt(
                        &#arg,
                        &mut #export::Formatter::with_args(
                            __stylish_formatter,
                            #formatter_args,
                            #style
                        ),
                    )?
                }
            }
        })
        .collect();
    let unused_positional_args =
        &HashSet::from_iter(0..num_positional_args) - &used_positional_args;
    let unused_named_args =
        &HashSet::from_iter(named_args_names.keys().cloned()) - &used_named_args;
    if !unused_positional_args.is_empty() || !unused_named_args.is_empty() {
        panic!("unused formatting arguments");
    }
    let implicit_named_args = quote! {
        (#(&#implicit_named_args_values,)*)
    };
    quote! {
        #export::Arguments {
            f: &match (#positional_args, #named_args_values, #implicit_named_args) {
                (__stylish_positional_args, __stylish_named_args, __stylish_implicit_named_args) => {
                    #[inline]
                    move |__stylish_formatter: &mut #export::Formatter| -> #export::fmt::Result {
                        #(#statements;)*
                        #export::fmt::Result::Ok(())
                    }
                }
            }
        }
    }
}

/// Internal implementation details of
/// [`stylish_core::format_args!`](https://docs.rs/stylish-core/latest/stylish_core/macro.format_args.html).
#[proc_macro]
pub fn format_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    format_args_impl(parse_macro_input!(input as ArgsInput))
        .into_token_stream()
        .into()
}

/// Internal implementation details of
/// [`stylish_core::format_args!`](https://docs.rs/stylish-core/latest/stylish_core/macro.format_args.html).
#[proc_macro]
pub fn format_args_nl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ArgsInput);
    let format = LitStr::new(&(input.format.value() + "\n"), input.format.span());
    format_args_impl(ArgsInput { format, ..input })
        .into_token_stream()
        .into()
}
