use crate::{Display, Formatter, Result};

#[doc(hidden)]
/// pub for macros
pub struct StdFmt<'a> {
    #[doc(hidden)]
    /// pub for macros
    pub f: &'a (dyn Fn(&mut core::fmt::Formatter<'_>) -> Result + 'a),
}

impl core::fmt::Display for StdFmt<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        (self.f)(f)
    }
}

impl core::fmt::Debug for StdFmt<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        (self.f)(f)
    }
}

#[doc(hidden)]
#[allow(missing_debug_implementations)]
/// pub for macros
pub struct StdFmtOther<'a>(
    #[doc(hidden)]
    /// pub for macros
    pub StdFmt<'a>,
);

#[doc(hidden)]
#[allow(missing_debug_implementations)]
/// pub for macros
pub struct StdFmtDebug<'a>(
    #[doc(hidden)]
    /// pub for macros
    pub StdFmt<'a>,
);

/// A precompiled version of a format string and its by-reference arguments.
///
/// Currently this can only be constructed via [`stylish::format_args!`], but it
/// may be possible to dynamically construct this at runtime in the future.
///
/// ```rust
/// let args = stylish::format_args!("{:(bg=red)} Will Robinson", "Danger");
/// assert_eq!(
///     stylish::html::format!("{:s}", args),
///     "<span style=background-color:red>Danger</span> Will Robinson",
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct Arguments<'a> {
    #[doc(hidden)]
    /// pub for macros
    pub f: &'a (dyn Fn(&mut Formatter<'_>) -> Result + 'a),
}

impl Display for StdFmtOther<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let arg = &self.0;
        std_write!(f, Other, arg)
    }
}

impl Display for StdFmtDebug<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let arg = &self.0;
        std_write!(f, Debug, arg)
    }
}

impl Display for Arguments<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (self.f)(f)
    }
}
