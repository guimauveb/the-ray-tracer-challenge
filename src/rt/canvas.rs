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

    pub const fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let index = self.get_pixel_index(x, y);
        self.pixels[index]
    }

    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // Bonus
    pub fn set_all_pixels_to_color(&mut self, color: Color) {
        for i in 0..self.pixels().len() {
            self.pixels[i] = color;
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

    // NOTE - Could certainly be improved -> Avoid creating "lines" (vec of string representing lines) then a final string
    fn split_lines_too_long(pixel_data: &str) -> String {
        // Create a vec with 1 string that will contain the split lines
        let mut lines: Vec<String> = vec![String::new()];

        let mut it_lines = pixel_data.split('\n').peekable();
        while let Some(line) = it_lines.next() {
            let mut it_colors = line.split(' ').peekable();
            while let Some(color) = it_colors.next() {
                let last_line_index = lines.len() - 1;
                let last_line_length = lines[last_line_index].len();

                lines[last_line_index].push_str(color);

                if let Some(next_color) = it_colors.peek() {
                    // can_insert_next_color_into_line is true if we can insert a space and the next color without exceeding 70 chars
                    // If true, insert a space, else insert a new line.
                    let can_insert_next_color_into_line =
                        (last_line_length + color.len() + 1 + next_color.len())
                            < PPM_MAX_CHARACTERS_PER_LINE;
                    if can_insert_next_color_into_line {
                        lines[last_line_index].push(' ');
                    } else {
                        lines.push(String::new());
                    }
                }
            }

            if it_lines.peek().is_some() {
                lines.push(String::new());
            }
        }

        // Join the vector of split lines into a final string
        // TODO - Map directly pixel_data to the "final" string result and avoid the Vec<String> of lines
        lines
            .iter()
            .enumerate()
            .map(|(i, s)| {
                return if i < lines.len() - 1 {
                    s.to_string() + "\n"
                } else {
                    s.to_string()
                };
            })
            .collect()
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

        // Some image softwares won't read PPM with lines that are more than 70 characters long.
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

#[test]
fn canvas_is_of_correct_size() {
    let canvas = Canvas::new(16, 8);
    assert_eq!(canvas.pixels().len(), 16 * 8);
}

#[test]
fn canvas_pixels_are_black_by_default() {
    let canvas = Canvas::new(16, 8);
    let black = Color::black();
    for &pixel in canvas.pixels() {
        assert_eq!(pixel, black);
    }
}

#[test]
fn pixel_at_returns_expected_color() {
    let mut canvas = Canvas::new(16, 8);
    let red = Color::new(1.0, 0.0, 0.0);
    canvas.write_pixel(2, 3, red);
    assert_eq!(canvas.pixel_at(2, 3), red);
}

#[test]
fn construct_ppm_header() {
    let canvas = Canvas::new(5, 3);
    let ppm = canvas.to_ppm();
    let ppm_header_lines: Vec<&str> = ppm.header().split("\n").collect();
    let expected_ppm_header_lines = vec!["P3", "5 3", "255", ""];
    assert_eq!(ppm_header_lines, expected_ppm_header_lines);
}

#[test]
fn construct_ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    let (c1, c2, c3) = (
        Color::new(1.5, 0.0, 0.0),
        Color::new(0.0, 0.5, 0.0),
        Color::new(-0.5, 0.0, 1.0),
    );
    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let ppm = canvas.to_ppm();
    let pixel_data_lines: Vec<&str> = ppm.pixel_data().split("\n").collect();
    let expected_pixel_data_lines = vec![
        "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
        "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
        "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        "",
    ];
    assert_eq!(pixel_data_lines, expected_pixel_data_lines);
}

#[test]
fn construct_ppm_pixel_data_lines_too_long_are_split() {
    let mut canvas = Canvas::new(10, 2);
    let color = Color::new(1.0, 0.8, 0.6);

    canvas.set_all_pixels_to_color(color);

    let ppm = canvas.to_ppm();
    let pixel_data_lines: Vec<&str> = ppm.pixel_data().split("\n").collect();
    let expected_pixel_data_lines = vec![
        "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
        "153 255 204 153 255 204 153 255 204 153 255 204 153",
        "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
        "153 255 204 153 255 204 153 255 204 153 255 204 153",
        "",
    ];
    assert_eq!(pixel_data_lines, expected_pixel_data_lines);
}

#[test]
fn ppm_files_are_terminated_by_a_newline_character() {
    let canvas = Canvas::new(5, 3);
    let ppm = canvas.to_ppm();
    let pixel_data_lines: Vec<&str> = ppm.pixel_data().split("\n").collect();
    let last_line = pixel_data_lines[pixel_data_lines.len() - 1];
    let expected_last_line = "";
    assert_eq!(last_line, expected_last_line);
}
