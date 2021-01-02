use stylish::{Argument, Arguments, Formatter};

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T: std::fmt::Debug> Debug for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(&Arguments {
            pieces: &[Argument::StdDebug(self)],
        })?;
        Ok(())
    }
}
