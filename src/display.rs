use stylish::Formatter;

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T: std::fmt::Display> Display for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())?;
        Ok(())
    }
}
