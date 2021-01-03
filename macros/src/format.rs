use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, none_of},
    combinator::{all_consuming, map, map_res, opt, recognize, rest, value, verify},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;

pub(crate) fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Default, Clone, Copy)]
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

impl<'a> FormatterArgs<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, alternate) = opt(value(true, tag("#")))(input)?;
        let (input, color) = opt(delimited(tag("<"), identifier, tag(">")))(input)?;
        Ok((
            input,
            FormatterArgs {
                alternate: alternate.unwrap_or_default(),
                restyle: Restyle { color },
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatArgRef<'a> {
    Next,
    Positional(usize),
    Named(&'a str),
}

impl<'a> Default for FormatArgRef<'a> {
    fn default() -> Self {
        Self::Next
    }
}

impl<'a> FormatArgRef<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(map_res(digit1, usize::from_str), FormatArgRef::Positional),
            map(identifier, FormatArgRef::Named),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Variant {
    Display,
    Debug,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Display
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FormatArg<'a> {
    pub(crate) arg: FormatArgRef<'a>,
    pub(crate) variant: Variant,
    pub(crate) args: FormatterArgs<'a>,
}

impl<'a> FormatArg<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, arg) = opt(FormatArgRef::parse)(input)?;
        let (input, args) = opt(preceded(tag(":"), FormatterArgs::parse))(input)?;
        let (input, variant) = opt(value(Variant::Debug, tag("?")))(input)?;
        Ok((
            input,
            Self {
                arg: arg.unwrap_or_default(),
                variant: variant.unwrap_or_default(),
                args: args.unwrap_or_default(),
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Piece<'a> {
    Lit(&'a str),
    Arg(FormatArg<'a>),
}

impl<'a> Piece<'a> {
    pub(crate) fn parse_lit(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(many1(alt((value((), none_of("{")), value((), tag("{{")))))),
            Self::Lit,
        )(input)
    }

    pub(crate) fn parse_final_lit(input: &'a str) -> IResult<&str, Self> {
        map(verify(rest, |s: &str| !s.is_empty()), Self::Lit)(input)
    }

    pub(crate) fn parse_arg(input: &'a str) -> IResult<&str, Self> {
        map(delimited(tag("{"), FormatArg::parse, tag("}")), Self::Arg)(input)
    }

    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_arg, Self::parse_lit))(input)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Format<'a> {
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl<'a> Format<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        all_consuming(map(
            pair(many0(Piece::parse), opt(Piece::parse_final_lit)),
            |(mut pieces, last)| {
                if let Some(last) = last {
                    pieces.push(last);
                }
                Self { pieces }
            },
        ))(input)
    }
}
