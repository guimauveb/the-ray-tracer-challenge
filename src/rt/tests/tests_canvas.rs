#[cfg(test)]
use crate::rt::{canvas::Canvas, color::Color, to_ppm::ToPPM};

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
    assert_eq!(canvas.pixel_at(2, 3), &red);
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
