use stylish::{Formatter, Result};

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T: std::fmt::Display> Display for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.to_string())?;
        Ok(())
    }
}
