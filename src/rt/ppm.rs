use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Ppm {
    header: String,
    pixel_data: String,
}

impl Ppm {
    pub fn new(
        identifier: &str,
        width: &str,
        height: &str,
        max_color_value: &str,
        pixel_data: String,
    ) -> Self {
        // TODO - Check for reallocations!
        let mut header = String::with_capacity(
            identifier.len()
                + "\n".len()
                + width.len()
                + "\n".len()
                + height.len()
                + "\n".len()
                + max_color_value.len(),
        );
        header.push_str(identifier);
        header.push('\n');
        header.push_str(width);
        header.push(' ');
        header.push_str(height);
        header.push('\n');
        header.push_str(max_color_value);

        Self { header, pixel_data }
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn pixel_data(&self) -> &str {
        &self.pixel_data
    }
}

impl Display for Ppm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.header, self.pixel_data)
    }
}
