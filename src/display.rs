use stylish::{Argument, Arguments, Formatter};

pub trait Display {
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
