use {
    super::{color::Color, ppm::Ppm},
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
    pub fn new(point: Point2d, color: Color) -> Self {
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
        // TODO - Check for reallocations!
        let mut pixels = Vec::<Pixel>::with_capacity(width * height);
        let default_color = Color::black();

        for i in 0..width {
            for j in 0..height {
                let point = Point2d::new(i, j);
                let pixel = Pixel::new(point, default_color);
                pixels.push(pixel);
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    // Might not be needed
    pub fn pixels(&self) -> &Vec<Pixel> {
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

    pub fn to_ppm(&self) -> Ppm {
        // TODO - String::with_capacity to avoid reallocations (compute capacity: char (colors + spaces + new lines)
        let mut pixel_data = String::new();

        // TODO - "process_color" (*MAX_COLOR_VALUE . clamp . ceil etc)
        for y in 0..self.height {
            for x in 0..self.width {
                pixel_data.push_str(
                    &((self.pixel_at(x, y).red() * MAX_COLOR_VALUE)
                        .clamp(MIN_COLOR_VALUE, MAX_COLOR_VALUE)
                        .ceil() as usize)
                        .to_string(),
                );
                pixel_data.push(' ');
                pixel_data.push_str(
                    &((self.pixel_at(x, y).green() * MAX_COLOR_VALUE)
                        .clamp(MIN_COLOR_VALUE, MAX_COLOR_VALUE)
                        .ceil() as usize)
                        .to_string(),
                );
                pixel_data.push(' ');
                pixel_data.push_str(
                    &((self.pixel_at(x, y).blue() * MAX_COLOR_VALUE)
                        .clamp(MIN_COLOR_VALUE, MAX_COLOR_VALUE)
                        .ceil() as usize)
                        .to_string(),
                );

                // If we haven't reached the end of the line, insert a space.
                if x != self.width - 1 {
                    pixel_data.push(' ');
                }
            }
            pixel_data.push('\n');
        }

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

// TODO - Update
//#[test]
//fn construct_ppm_header() {
//    let canvas = Canvas::new(5, 3);
//    let ppm = canvas.to_ppm();
//    let ppm_header_lines: Vec<&str> = ppm.header().split("\n").collect();
//    let expected_ppm_header_lines = vec!["P3", "5 3", "255"];
//    assert_eq!(ppm_header_lines, expected_ppm_header_lines);
//}

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
    println!("{:#?}", &pixel_data_lines);
}

#[test]
fn construct_ppm_pixel_data_max_char_per_line() {
    let mut canvas = Canvas::new(10, 2);
    let color = Color::new(1.0, 0.8, 0.6);

    canvas.set_all_pixels_to_color(color);

    let ppm = canvas.to_ppm();
    let pixel_data_lines: Vec<&str> = ppm.pixel_data().split("\n").collect();
    println!("{:#?}", &pixel_data_lines);
    //let expected_pixel_data_lines = vec![
    //    "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ",
    //    "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
    //    "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
    //];
    //assert_eq!(pixel_data_lines, expected_pixel_data_lines);
}

#[test]
fn ppm_files_are_terminated_with_a_newline() {
    // TODO
}
