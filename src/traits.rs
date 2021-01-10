use stylish::{Argument, Arguments, Formatter};

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait Octal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait LowerHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait UpperHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait Pointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait LowerExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub trait UpperExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T: std::fmt::Display> Display for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdDisplay(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::Debug> Debug for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdDebug(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::Octal> Octal for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdOctal(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::LowerHex> LowerHex for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdLowerHex(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::UpperHex> UpperHex for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdUpperHex(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::Pointer> Pointer for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdPointer(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::Binary> Binary for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdBinary(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::LowerExp> LowerExp for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdLowerExp(self)],
        })?;
        Ok(())
    }
}

impl<T: std::fmt::UpperExp> UpperExp for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdUpperExp(self)],
        })?;
        Ok(())
    }
}
