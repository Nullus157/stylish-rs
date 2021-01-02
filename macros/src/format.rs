use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    multi::many0,
    IResult,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug)]
pub(crate) struct Restyle<'a> {
    color: Option<&'a str>,
}

impl<'a> ToTokens for Restyle<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Restyle { color } = self;
        let color = match color {
            Some("blue") => quote!(&stylish::style::Color::Blue),
            Some(color) => panic!("Unknown color {}", color),
            None => quote!(&()),
        };
        (quote! { #color }).to_tokens(tokens)
    }
}

#[derive(Debug)]
pub(crate) struct FormatterArgs<'a> {
    alternate: bool,
    restyle: Restyle<'a>,
}

impl<'a> ToTokens for FormatterArgs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FormatterArgs { alternate, restyle } = self;
        (quote! {
            stylish::FormatterArgs {
                alternate: #alternate,
                restyle: #restyle,
            }
        })
        .to_tokens(tokens)
    }
}

#[derive(Debug)]
pub(crate) enum Variant<'a> {
    Display(FormatterArgs<'a>),
    Debug(FormatterArgs<'a>),
}

#[derive(Debug)]
pub(crate) struct FormatArg<'a> {
    pub(crate) variant: Variant<'a>,
}

#[derive(Debug)]
pub(crate) enum Piece<'a> {
    Lit(&'a str),
    Arg(FormatArg<'a>),
}

#[derive(Debug)]
pub(crate) struct Format<'a> {
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl<'a> Piece<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(tag("{}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Display(FormatterArgs {
                        alternate: false,
                        restyle: Restyle { color: None },
                    }),
                })
            }),
            map(tag("{:<blue>}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Display(FormatterArgs {
                        alternate: false,
                        restyle: Restyle {
                            color: Some("blue"),
                        },
                    }),
                })
            }),
            map(tag("{:?}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Debug(FormatterArgs {
                        alternate: false,
                        restyle: Restyle { color: None },
                    }),
                })
            }),
            map(tag("{:#?}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Debug(FormatterArgs {
                        alternate: false,
                        restyle: Restyle { color: None },
                    }),
                })
            }),
            map(take_until("{"), Self::Lit),
        ))(input)
    }
}

impl<'a> Format<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        map(many0(Piece::parse), |pieces| Self { pieces })(input)
    }
}
