macro_rules! std_write {
    (@str $f:ident $val:ident [] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {{
        use core::fmt::Write;
        match $f.format {
            $(
                crate::formatter::FormatterArgs { $($field : $pat,)* } => {
                    core::write!(crate::std_compat::StdProxy($f), concat!("{:", $($s,)* "}"), *$val, $($($arg)*,)*)
                }
            )*
        }
    }};

    (@str $f:ident $val:ident [precision $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ precision: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ precision: Some(precision), $($field : $pat,)* } => ((".precision$", $($s,)*), (precision=precision), $(($($arg)*),)*);)*
        })
    };

    (@str $f:ident $val:ident [width $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ width: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ width: Some(width), $($field : $pat,)* } => (("width$", $($s,)*), (width=width), $(($($arg)*),)*);)*
        })
    };

    (@str $f:ident $val:ident [zero $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ zero: false, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ zero: true, $($field : $pat,)* } => (("0", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $f:ident $val:ident [alternate $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ alternate: false, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ alternate: true, $($field : $pat,)* } => (("#", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $f:ident $val:ident [sign $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ sign: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ sign: Some(crate::formatter::Sign::Plus), $($field : $pat,)* } => (("+", $($s,)*), $(($($arg)*),)*);)*
            $({ sign: Some(crate::formatter::Sign::Minus), $($field : $pat,)* } => (("-", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    (@str $f:ident $val:ident [align $($flag:ident)*] { $({ $($field:ident : $pat:pat,)* } => (($($s:literal,)*), $(($($arg:tt)*),)*);)* }) => {
        std_write!(@str $f $val [$($flag)*] {
            $({ align: None, $($field : $pat,)* } => (($($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(crate::formatter::Align::Left), $($field : $pat,)* } => (("<", $($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(crate::formatter::Align::Center), $($field : $pat,)* } => (("^", $($s,)*), $(($($arg)*),)*);)*
            $({ align: Some(crate::formatter::Align::Right), $($field : $pat,)* } => ((">", $($s,)*), $(($($arg)*),)*);)*
        })
    };

    // For most traits we can pipe them through `Display` since they have the same function
    // signature
    (@str $f:ident $val:ident [Other $($flag:ident)*]) => {
        std_write!(@str $f $val [$($flag)*] { { debug_hex: _, } => (("",),); })
    };

    // But `Debug` is special as it has extra hex flags
    (@str $f:ident $val:ident [Debug $($flag:ident)*]) => {
        std_write!(@str $f $val [$($flag)*] {
            { debug_hex: None, } => (("?",),);
            { debug_hex: Some(crate::formatter::DebugHex::Lower), } => (("x?",),);
            { debug_hex: Some(crate::formatter::DebugHex::Upper), } => (("X?",),);
        })
    };

    ($f:ident, $trait:ident, $val:ident) => {
        std_write!(@str $f $val [$trait precision width zero alternate sign align])
    };
}

pub(crate) struct StdProxy<'a, 'b>(pub(crate) &'a mut crate::formatter::Formatter<'b>);

impl<'a, 'b> core::fmt::Write for StdProxy<'a, 'b> {
    fn write_str(&mut self, s: &str) -> crate::Result {
        self.0.write_str(s)
    }
}
