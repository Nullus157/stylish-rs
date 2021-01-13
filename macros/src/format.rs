use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, anychar, digit1, none_of},
    combinator::{all_consuming, cut, map, map_res, opt, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum Align {
    Left,
    Center,
    Right,
}

impl ToTokens for Align {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Left => quote!(stylish::Align::Left),
            Self::Center => quote!(stylish::Align::Center),
            Self::Right => quote!(stylish::Align::Right),
        }
        .to_tokens(tokens)
    }
}

impl Align {
    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Left, tag("<")),
            value(Self::Center, tag("^")),
            value(Self::Right, tag(">")),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Sign {
    Plus,
    Minus,
}

impl ToTokens for Sign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Plus => quote!(stylish::Sign::Plus),
            Self::Minus => quote!(stylish::Sign::Minus),
        }
        .to_tokens(tokens)
    }
}

impl Sign {
    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(Self::Plus, tag("+")), value(Self::Minus, tag("-"))))(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum DebugHex {
    Lower,
    Upper,
}

impl ToTokens for DebugHex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lower => quote!(stylish::DebugHex::Lower),
            Self::Upper => quote!(stylish::DebugHex::Upper),
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct FormatterArgs<'a> {
    align: Option<Align>,
    sign: Option<Sign>,
    alternate: bool,
    zero: bool,
    width: Option<Count<'a>>,
    precision: Option<Count<'a>>,
    debug_hex: Option<DebugHex>,
    restyle: Restyle<'a>,
}

fn quote_opt<T: ToTokens>(value: &Option<T>) -> TokenStream {
    match value {
        Some(value) => quote!(core::option::Option::Some(#value)),
        None => quote!(core::option::Option::None),
    }
}

impl<'a> ToTokens for FormatterArgs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FormatterArgs {
            align,
            sign,
            alternate,
            zero,
            width,
            precision,
            debug_hex,
            restyle,
        } = self;
        let align = quote_opt(align);
        let sign = quote_opt(sign);
        let width = quote_opt(width);
        let precision = quote_opt(precision);
        let debug_hex = quote_opt(debug_hex);
        (quote! {
            stylish::FormatterArgs {
                align: #align,
                sign: #sign,
                alternate: #alternate,
                zero: #zero,
                width: #width,
                precision: #precision,
                debug_hex: #debug_hex,
                restyle: #restyle,
            }
        })
        .to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatTrait {
    Display,
    Debug,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}

impl Default for FormatTrait {
    fn default() -> Self {
        Self::Display
    }
}

impl ToTokens for FormatTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FormatTrait::Display => {
                quote!(stylish::Display)
            }
            FormatTrait::Debug => {
                quote!(stylish::Debug)
            }
            FormatTrait::Octal => {
                quote!(stylish::Octal)
            }
            FormatTrait::LowerHex => {
                quote!(stylish::LowerHex)
            }
            FormatTrait::UpperHex => {
                quote!(stylish::UpperHex)
            }
            FormatTrait::Pointer => {
                quote!(stylish::Pointer)
            }
            FormatTrait::Binary => {
                quote!(stylish::Binary)
            }
            FormatTrait::LowerExp => {
                quote!(stylish::LowerExp)
            }
            FormatTrait::UpperExp => {
                quote!(stylish::UpperExp)
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Count<'a> {
    Parameter(FormatArgRef<'a>),
    Integer(usize),
}

impl<'a> Count<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(terminated(FormatArgRef::parse, tag("$")), Self::Parameter),
            map(map_res(digit1, usize::from_str), Self::Integer),
        ))(input)
    }
}

impl<'a> ToTokens for Count<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Count::Parameter(_) => todo!(),
            Count::Integer(value) => quote!(&#value).to_tokens(tokens),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct FormatSpec<'a> {
    pub(crate) formatter_args: FormatterArgs<'a>,
    pub(crate) format_trait: FormatTrait,
}

impl<'a> FormatSpec<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, align) = opt(alt((
            pair(anychar, Align::parse),
            map(Align::parse, |align| (' ', align)),
        )))(input)?;
        let align = align.map(|(fill, align)| {
            if fill != ' ' {
                todo!()
            }
            align
        });
        let (input, sign) = opt(Sign::parse)(input)?;
        let (input, alternate) = opt(value(true, tag("#")))(input)?;
        let (input, zero) = opt(value(true, tag("0")))(input)?;
        let (input, width) = opt(Count::parse)(input)?;
        let (input, precision) = opt(preceded(tag("."), Count::parse))(input)?;
        let (input, color) = opt(delimited(tag("("), identifier, tag(")")))(input)?;
        let (input, debug_hex_and_format_trait) = opt(alt((
            value((None, FormatTrait::Debug), tag("?")),
            value((Some(DebugHex::Lower), FormatTrait::Debug), tag("x?")),
            value((Some(DebugHex::Upper), FormatTrait::Debug), tag("X?")),
            value((None, FormatTrait::Octal), tag("o")),
            value((None, FormatTrait::LowerHex), tag("x")),
            value((None, FormatTrait::UpperHex), tag("X")),
            value((None, FormatTrait::Pointer), tag("p")),
            value((None, FormatTrait::Binary), tag("b")),
            value((None, FormatTrait::LowerExp), tag("e")),
            value((None, FormatTrait::UpperExp), tag("E")),
        )))(input)?;
        let debug_hex = debug_hex_and_format_trait.and_then(|(debug_hex, _)| debug_hex);
        let format_trait = debug_hex_and_format_trait.map(|(_, format_trait)| format_trait);
        Ok((
            input,
            FormatSpec {
                formatter_args: FormatterArgs {
                    align,
                    sign,
                    alternate: alternate.unwrap_or_default(),
                    zero: zero.unwrap_or_default(),
                    width,
                    precision,
                    debug_hex,
                    restyle: Restyle { color },
                },
                format_trait: format_trait.unwrap_or_default(),
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatArgRef<'a> {
    Positional(usize),
    Named(&'a str),
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
pub(crate) struct FormatArg<'a> {
    pub(crate) arg: Option<FormatArgRef<'a>>,
    pub(crate) format_spec: FormatSpec<'a>,
}

impl<'a> FormatArg<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, arg) = opt(FormatArgRef::parse)(input)?;
        let (input, format_spec) = opt(preceded(tag(":"), FormatSpec::parse))(input)?;
        Ok((
            input,
            Self {
                arg,
                format_spec: format_spec.unwrap_or_default(),
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
        alt((
            map(recognize(many1(none_of("{}"))), Self::Lit),
            value(Self::Lit("{"), tag("{{")),
            value(Self::Lit("}"), tag("}}")),
        ))(input)
    }

    pub(crate) fn parse_arg(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(tag("{"), cut(FormatArg::parse), tag("}")),
            Self::Arg,
        )(input)
    }

    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_lit, Self::parse_arg))(input)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Format<'a> {
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl<'a> Format<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        all_consuming(map(many0(Piece::parse), |pieces| Self { pieces }))(input)
    }
}
