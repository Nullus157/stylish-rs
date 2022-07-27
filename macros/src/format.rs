use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, anychar, digit1, none_of},
    combinator::{all_consuming, cut, map, map_res, opt, recognize, value},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

#[derive(Debug, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

impl Color {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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

#[derive(Debug, Clone)]
pub enum Intensity {
    Normal,
    Bold,
    Faint,
}

impl Intensity {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Intensity::Normal, tag("normal")),
            value(Intensity::Bold, tag("bold")),
            value(Intensity::Faint, tag("faint")),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub enum Restyle {
    Foreground(Color),
    Background(Color),
    Intensity(Intensity),
}

impl Restyle {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                preceded(tag("fg"), cut(preceded(tag("="), Color::parse))),
                Restyle::Foreground,
            ),
            map(
                preceded(tag("bg"), cut(preceded(tag("="), Color::parse))),
                Restyle::Background,
            ),
            map(Intensity::parse, Restyle::Intensity),
        ))(input)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Restyles {
    // TODO: This should only allow each variant once, but there's no sort of optional-permutation
    // helper in nom
    pub restyles: Vec<Restyle>,
}

impl Restyles {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list0(tag(","), cut(Restyle::parse)), |restyles| {
            Restyles { restyles }
        })(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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

impl Sign {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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

impl<'a> Count<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(terminated(FormatArgRef::parse, tag("$")), Self::Parameter),
            map(map_res(digit1, usize::from_str), Self::Integer),
        ))(input)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FormatSpec<'a> {
    pub formatter_args: FormatterArgs<'a>,
    pub restyles: Restyles,
    pub format_trait: FormatTrait,
}

impl<'a> FormatSpec<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
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
        let (input, restyles) = opt(delimited(tag("("), cut(Restyles::parse), tag(")")))(input)?;
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
                restyles: restyles.unwrap_or_default(),
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

impl<'a> FormatArgRef<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
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

impl<'a> FormatArg<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
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

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_lit, Self::parse_arg))(input)
    }
}

#[derive(Debug, Clone)]
pub struct Format<'a> {
    pub pieces: Vec<Piece<'a>>,
}

impl<'a> Format<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        all_consuming(map(many0(Piece::parse), |pieces| Self { pieces }))(input)
    }
}
