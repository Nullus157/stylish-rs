use crate::{Formatter, Result};

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait Octal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait LowerHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait UpperHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait Pointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait LowerExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub trait UpperExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T: std::fmt::Display> Display for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_display(self)
    }
}

impl<T: std::fmt::Debug> Debug for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_debug(self)
    }
}

impl<T: std::fmt::Octal> Octal for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_octal(self)
    }
}

impl<T: std::fmt::LowerHex> LowerHex for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_lower_hex(self)
    }
}

impl<T: std::fmt::UpperHex> UpperHex for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_upper_hex(self)
    }
}

impl<T: std::fmt::Pointer> Pointer for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_pointer(self)
    }
}

impl<T: std::fmt::Binary> Binary for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_binary(self)
    }
}

impl<T: std::fmt::LowerExp> LowerExp for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_lower_exp(self)
    }
}

impl<T: std::fmt::UpperExp> UpperExp for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_std_upper_exp(self)
    }
}
