use {
    super::{
        color::Color,
        ppm::{Ppm, PPM_MAX_CHARACTERS_PER_LINE},
    },
    crate::primitive::point2d::Point2d,
};

const MIN_COLOR_VALUE: f64 = 0.0;
const MAX_COLOR_VALUE: f64 = 255.0;

#[derive(Debug)]
pub struct Pixel {
    point: Point2d,
    color: Color,
}

impl Pixel {
    pub const fn new(point: Point2d, color: Color) -> Self {
        Self { point, color }
    }
}

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::<Pixel>::with_capacity(width * height);
        //let initial_capacity = pixels.capacity();

        let default_color = Color::black();

        for i in 0..width {
            for j in 0..height {
                let point = Point2d::new(i, j);
                let pixel = Pixel::new(point, default_color);
                pixels.push(pixel);
            }
        }

        //let final_capacity = pixels.capacity();
        //assert_eq!(initial_capacity, final_capacity);

        Self {
            width,
            height,
            pixels,
        }
    }

    // Might not be needed
    pub const fn pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels
            .iter_mut()
            .find(|pixel| pixel.point.x == x && pixel.point.y == y)
            .unwrap_or_else(|| panic!("Pixel not found!"))
            .color = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels
            .iter()
            .find(|pixel| pixel.point.x == x && pixel.point.y == y)
            .unwrap_or_else(|| panic!("Pixel not found!"))
            .color
    }

    // Bonus
    pub fn set_all_pixels_to_color(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            pixel.color = color;
        }
    }

    fn build_ppm_pixel_data(&self) -> String {
        /* Transform the color value originally ranging from 0.0 to 1.0 to a value between 0.0 and 255.0.
         * Then take the smallest integer greater than the result.
         * Finally, convert it to a string.*/
        fn process_color_for_ppm(color: f64) -> String {
            ((color * MAX_COLOR_VALUE)
                .clamp(MIN_COLOR_VALUE, MAX_COLOR_VALUE)
                .ceil() as usize)
                .to_string()
        }

        fn split_ppm_lines_too_long(pixel_data: &str) -> String {
            /* The final PPM pixel_data in which we split lines greater than 70 chars will be the same length as the pixel_data, since we are only
             * replacing spaces by newlines. */
            let mut split_pixel_data = String::with_capacity(pixel_data.len());

            let lines: Vec<&str> = pixel_data.split('\n').collect();
            let line_count = lines.len();

            for (line_index, line) in lines.into_iter().enumerate() {
                for (i, c) in line.chars().enumerate() {
                    // Insert a newline if we arrive at a char which position is a multiple of 70.
                    if (i > 0) && (i % PPM_MAX_CHARACTERS_PER_LINE == 0) {
                        let mut j = i;
                        // To avoid splitting a number (pixel), we go back to the white space before it to insert a new line.
                        while line.chars().nth(j).unwrap().is_numeric() {
                            split_pixel_data.pop();
                            j -= 1;
                        }
                        // When we have found a whitespace, we insert a new line.
                        split_pixel_data.push('\n');
                        // Then, we insert what was after the white space (the one before the split number) and until the current iterated char (included).
                        split_pixel_data.push_str(&pixel_data[(j + 1)..=i]);
                    } else {
                        split_pixel_data.push(c);
                    }
                }
                // Insert a new line unless we've arrived at the last line.
                if line_index < (line_count - 1) {
                    split_pixel_data.push('\n');
                }
            }

            split_pixel_data
        }

        // TODO - String::with_capacity to avoid reallocations (compute capacity: char (colors + spaces + new lines) -> take maximum possible size of pixel_data.
        let mut pixel_data = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                pixel_data.push_str(&process_color_for_ppm(self.pixel_at(x, y).red()));
                pixel_data.push(' ');
                pixel_data.push_str(&process_color_for_ppm(self.pixel_at(x, y).green()));
                pixel_data.push(' ');
                pixel_data.push_str(&process_color_for_ppm(self.pixel_at(x, y).blue()));

                // If we haven't reached the end of the line, insert a space.
                if x < self.width - 1 {
                    pixel_data.push(' ');
                }
            }
            pixel_data.push('\n');
        }

        // Some image softwares won't read PPM with lines that are more than 70 characters long.
        split_ppm_lines_too_long(&pixel_data)
    }

    pub fn to_ppm(&self) -> Ppm {
        let pixel_data = self.build_ppm_pixel_data();
        Ppm::new(
            "P3",
            &self.width.to_string(),
            &self.height.to_string(),
            &MAX_COLOR_VALUE.to_string(),
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
    for pixel in canvas.pixels() {
        assert_eq!(pixel.color, Color::black());
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
