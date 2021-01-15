use crate::{Display, Formatter, Result, Style, Write};

pub struct String {
    string: std::string::String,
    styles: Vec<(usize, Style)>,
}

impl Write for String {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        if Some(style) != self.styles.last().map(|&(_, style)| style) {
            self.styles.push((self.string.len(), style));
        }
        self.string.push_str(s);
        Ok(())
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut styles = self.styles.iter().peekable();
        while let Some(&(start, style)) = styles.next() {
            let end = styles
                .peek()
                .map(|&&(end, _)| end)
                .unwrap_or_else(|| self.string.len());
            f.with(style).write_str(&self.string[start..end])?;
        }
        Ok(())
    }
}
