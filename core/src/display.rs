use crate::{Formatter, Result};

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T: Display + ?Sized> Display for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&**self).fmt(f)
    }
}
