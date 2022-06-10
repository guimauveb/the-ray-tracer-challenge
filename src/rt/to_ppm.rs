use super::ppm::Ppm;

pub trait ToPPM {
    // TODO - PPM type, PPM_MAX_CHARACTERS_PER_LINE, PPM_MIN_COLOR_VALUE, PPM_MAX_COLOR_VALUE
    fn build_pixel_data(&self) -> String;

    /* Transform the color value originally ranging from 0.0 to 1.0 to a value between 0.0 and 255.0.
     * Then take the smallest integer greater than the result.
     * Finally, convert it to a string.*/
    fn process_color(color: f32) -> String;

    /* NOTE (good to know if we get rid a the Vec<String>)
     * The final PPM pixel_data in which we split lines greater than 70 chars will be the same length as the pixel_data, since we are only
     * replacing spaces by newlines. */
    fn split_lines_too_long(pixel_data: &str) -> String;
    fn to_ppm(&self) -> Ppm;
}
