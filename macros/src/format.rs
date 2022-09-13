use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, none_of, satisfy},
    combinator::{all_consuming, cut, map, map_res, opt, recognize, value},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use stylish_style::{Background, Color, Foreground, Intensity, Restyle, Style, StyleDiff};

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((satisfy(unicode_ident::is_xid_start), char('_'))),
        many0(satisfy(unicode_ident::is_xid_continue)),
    ))(input)
}

pub trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> IResult<&str, Self>;
}

impl<'a> Parse<'a> for Color {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            value(Color::Black, tag("black")),
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Yellow, tag("yellow")),
            value(Color::Blue, tag("blue")),
            value(Color::Magenta, tag("magenta")),
            value(Color::Cyan, tag("cyan")),
            value(Color::White, tag("white")),
            value(Color::Default, tag("default")),
        ))(input)
    }
}

impl<'a> Parse<'a> for Intensity {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            value(Intensity::Normal, tag("normal")),
            value(Intensity::Bold, tag("bold")),
            value(Intensity::Faint, tag("faint")),
        ))(input)
    }
}

impl<'a> Parse<'a> for Box<dyn Restyle> {
    #[allow(trivial_casts)]
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(
                preceded(tag("fg"), cut(preceded(tag("="), Color::parse))),
                |color| Box::new(Foreground(color)) as _,
            ),
            map(
                preceded(tag("bg"), cut(preceded(tag("="), Color::parse))),
                |color| Box::new(Background(color)) as _,
            ),
            map(Intensity::parse, |intensity| Box::new(intensity) as _),
        ))(input)
    }
}

impl<'a> Parse<'a> for StyleDiff {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // TODO: This should only allow each variant once, but there's no sort of
        // optional-permutation helper in nom
        map(
            separated_list0(tag(","), cut(<Box<dyn Restyle>>::parse)),
            |restyles| {
                Style::default()
                    .with(restyles.as_slice())
                    .diff_from(Style::default())
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl<'a> Parse<'a> for Align {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            value(Self::Left, tag("<")),
            value(Self::Center, tag("^")),
            value(Self::Right, tag(">")),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

impl<'a> Parse<'a> for Sign {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((value(Self::Plus, tag("+")), value(Self::Minus, tag("-"))))(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DebugHex {
    Lower,
    Upper,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FormatterArgs<'a> {
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero: bool,
    pub width: Option<Count<'a>>,
    pub precision: Option<Count<'a>>,
    pub debug_hex: Option<DebugHex>,
}

#[derive(Debug, Clone, Copy)]
pub enum FormatTrait {
    Display,
    Debug,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
    Stylish,
}

impl Default for FormatTrait {
    fn default() -> Self {
        Self::Display
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Count<'a> {
    Parameter(FormatArgRef<'a>),
    Integer(usize),
}

impl<'a> Parse<'a> for Count<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(terminated(FormatArgRef::parse, tag("$")), Self::Parameter),
            map(map_res(digit1, usize::from_str), Self::Integer),
        ))(input)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FormatSpec<'a> {
    pub formatter_args: FormatterArgs<'a>,
    pub style: StyleDiff,
    pub format_trait: FormatTrait,
}

impl<'a> Parse<'a> for FormatSpec<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
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
        let (input, style) = opt(delimited(tag("("), cut(StyleDiff::parse), tag(")")))(input)?;
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
            value((None, FormatTrait::Stylish), tag("s")),
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
                },
                style: style.unwrap_or_default(),
                format_trait: format_trait.unwrap_or_default(),
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FormatArgRef<'a> {
    Positional(usize),
    Named(&'a str),
}

impl<'a> Parse<'a> for FormatArgRef<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(map_res(digit1, usize::from_str), FormatArgRef::Positional),
            map(identifier, FormatArgRef::Named),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct FormatArg<'a> {
    pub arg: Option<FormatArgRef<'a>>,
    pub format_spec: FormatSpec<'a>,
}

impl<'a> Parse<'a> for FormatArg<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
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

#[derive(Debug, Clone)]
#[allow(variant_size_differences)]
pub enum Piece<'a> {
    Lit(&'a str),
    Arg(FormatArg<'a>),
}

impl<'a> Piece<'a> {
    pub fn parse_lit(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(recognize(many1(none_of("{}"))), Self::Lit),
            value(Self::Lit("{"), tag("{{")),
            value(Self::Lit("}"), tag("}}")),
        ))(input)
    }

    pub fn parse_arg(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(tag("{"), cut(FormatArg::parse), tag("}")),
            Self::Arg,
        )(input)
    }
}

impl<'a> Parse<'a> for Piece<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_lit, Self::parse_arg))(input)
    }
}

#[derive(Debug, Clone)]
pub struct Format<'a> {
    pub pieces: Vec<Piece<'a>>,
}

impl<'a> Parse<'a> for Format<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        all_consuming(map(many0(Piece::parse), |pieces| Self { pieces }))(input)
    }
}
