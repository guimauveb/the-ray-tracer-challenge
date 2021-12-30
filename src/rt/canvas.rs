use super::color::Color;

const PPM_MAX_CHARACTER_PER_LINE: usize = 70;

#[derive(Debug)]
pub struct Point2d {
    x: i64,
    y: i64,
}

impl Point2d {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

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

#[derive(Debug, PartialEq)]
pub struct PpmHeader {
    identifier: String,
    width_and_height: String,
    max_color_value: i64,
}

impl PpmHeader {
    pub fn new(identifier: String, width: String, height: String, max_color_value: i64) -> Self {
        Self {
            identifier,
            width_and_height: width + " " + &height,
            max_color_value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ppm {
    header: PpmHeader,
    pixel_data: String,
}

impl Ppm {
    pub fn new(header: PpmHeader, pixel_data: String) -> Self {
        Self { header, pixel_data }
    }

    pub fn header(&self) -> &PpmHeader {
        &self.header
    }

    pub fn pixel_data(&self) -> &str {
        &self.pixel_data
    }
}

#[derive(Debug)]
pub struct Canvas {
    width: i64,
    height: i64,
    pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: i64, height: i64) -> Self {
        let mut pixels = vec![];

        for i in 0..width {
            for j in 0..height {
                let point = Point2d::new(i, j);
                let color = Color::black();
                let pixel = Pixel::new(point, color);
                pixels.push(pixel);
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: i64, y: i64, color: Color) {
        let pixel_index = self
            .pixels
            .iter()
            .position(|pixel| pixel.point.x == x && pixel.point.y == y);
        match pixel_index {
            Some(index) => {
                self.pixels[index].point.x = x;
                self.pixels[index].point.y = y;
                self.pixels[index].color = color;
            }
            None => {
                // Should panic?
                println!("Could not find pixel at position: x = {}, y = {}", x, y);
            }
        }
    }
    pub fn pixel_at(&self, x: i64, y: i64) -> Option<Color> {
        let pixel_result = self
            .pixels
            .iter()
            .find(|pixel| pixel.point.x == x && pixel.point.y == y);

        match pixel_result {
            Some(pixel) => Some(pixel.color),
            None => None,
        }
    }

    pub fn to_ppm(&self) -> Ppm {
        let ppm_header = PpmHeader::new(
            "P3".to_string(),
            self.width.to_string(),
            self.height.to_string(),
            255,
        );

        fn is_end_of_line(pixel_data_length: usize) -> bool {
            (pixel_data_length > 0) && (pixel_data_length % PPM_MAX_CHARACTER_PER_LINE == 0)
        }

        fn build_pixel_data(color: f64, pixel_data: &mut String) -> &mut String {
            if is_end_of_line(pixel_data.len()) {
                pixel_data.push_str("\n");
            }
            pixel_data.push_str(&color.to_string());

            if is_end_of_line(pixel_data.len()) {
                pixel_data.push_str("\n");
            } else {
                pixel_data.push_str(" ");
            }

            pixel_data
        }

        // TODO - with_capacity (compute capacity: char (colors + spaces + new lines)
        let mut pixel_data = String::new();

        for pixel in self.pixels() {
            build_pixel_data(pixel.color.red(), &mut pixel_data);
            build_pixel_data(pixel.color.green(), &mut pixel_data);
            build_pixel_data(pixel.color.blue(), &mut pixel_data);
        }

        println!("{}", &pixel_data);

        Ppm::new(ppm_header, pixel_data)
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
    assert_eq!(canvas.pixel_at(2, 3).unwrap(), red);
}

#[test]
fn construct_ppm_header() {
    let canvas = Canvas::new(5, 3);
    let ppm = canvas.to_ppm();
    let expected_ppm_header =
        PpmHeader::new("P3".to_string(), "5".to_string(), "3".to_string(), 255);
    assert_eq!(ppm.header(), &expected_ppm_header);
}

#[test]
fn canvas_to_ppm() {
    let canvas = Canvas::new(16, 8);
    let ppm = canvas.to_ppm();
    //println!("{}", ppm.pixel_data());
}
