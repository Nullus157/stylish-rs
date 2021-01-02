use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    multi::many0,
    IResult,
};

pub(crate) enum Variant {
    Display,
    Debug(bool),
}

pub(crate) struct FormatArg {
    pub(crate) variant: Variant,
}

pub(crate) enum Piece<'a> {
    Lit(&'a str),
    Arg(FormatArg),
}

pub(crate) struct Format<'a> {
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl<'a> Piece<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(tag("{}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Display,
                })
            }),
            map(tag("{:?}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Debug(false),
                })
            }),
            map(tag("{:#?}"), |_| {
                Self::Arg(FormatArg {
                    variant: Variant::Debug(true),
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
