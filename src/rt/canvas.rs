use super::{
    color::Color,
    ppm::{Ppm, PPM_MAX_CHARACTERS_PER_LINE, PPM_MAX_COLOR_VALUE, PPM_MIN_COLOR_VALUE},
    to_ppm::ToPPM,
};

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    const fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        let index = self.get_pixel_index(x, y);
        &self.pixels[index]
    }

    // Bonus
    pub fn set_all_pixels_to_color(&mut self, color: Color) {
        for i in 0..self.pixels().len() {
            self.pixels[i] = color.clone();
        }
    }
}

impl ToPPM for Canvas {
    fn process_color(color: f64) -> String {
        ((color * PPM_MAX_COLOR_VALUE)
            .clamp(PPM_MIN_COLOR_VALUE, PPM_MAX_COLOR_VALUE)
            .ceil() as usize)
            .to_string()
    }

    fn split_lines_too_long(pixel_data: &str) -> String {
        let mut cleaned_pixel_data = String::with_capacity(pixel_data.len());
        let mut it_lines = pixel_data.split('\n').peekable();
        let mut last_line_start_index: usize = 0;

        while let Some(line) = it_lines.next() {
            let mut it_colors = line.split(' ').peekable();
            while let Some(color) = it_colors.next() {
                let last_line_length = cleaned_pixel_data[last_line_start_index..].len();
                cleaned_pixel_data.push_str(color);
                if let Some(next_color) = it_colors.peek() {
                    // can_insert_next_color_into_line is true if we can insert a space and the next color without exceeding 70 chars
                    // If true, insert a space, else insert a new line.
                    let can_insert_next_color_into_line =
                        (last_line_length + color.len() + 1 + next_color.len())
                            < PPM_MAX_CHARACTERS_PER_LINE;
                    if can_insert_next_color_into_line {
                        cleaned_pixel_data.push(' ');
                    } else {
                        cleaned_pixel_data.push('\n');
                        last_line_start_index = cleaned_pixel_data.len() - 1;
                    }
                }
            }
            if it_lines.peek().is_some() {
                cleaned_pixel_data.push('\n');
                last_line_start_index = cleaned_pixel_data.len() - 1;
            }
        }

        cleaned_pixel_data
    }

    fn build_pixel_data(&self) -> String {
        // TODO - String::with_capacity to avoid reallocations (compute capacity: char (colors + spaces + new lines) -> take maximum possible size of pixel_data?
        let mut pixel_data = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_pixel_index(x, y);
                pixel_data.push_str(&Self::process_color(self.pixels[index].red()));
                pixel_data.push(' ');
                pixel_data.push_str(&Self::process_color(self.pixels[index].green()));
                pixel_data.push(' ');
                pixel_data.push_str(&Self::process_color(self.pixels[index].blue()));

                // If we haven't reached the end of the line, insert a space.
                if x < self.width - 1 {
                    pixel_data.push(' ');
                }
            }
            pixel_data.push('\n');
        }

        Self::split_lines_too_long(&pixel_data)
    }

    fn to_ppm(&self) -> Ppm {
        let pixel_data = self.build_pixel_data();
        Ppm::new(
            "P3",
            &self.width.to_string(),
            &self.height.to_string(),
            &PPM_MAX_COLOR_VALUE.to_string(),
            pixel_data,
        )
    }
}
