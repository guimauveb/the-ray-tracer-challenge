use {
    super::{matrix::Matrix, ray::Ray},
    crate::tuple::{point::Point, vector::Vector},
};
/// Note: Pixel sizes are of type `f64`, even though they will always be postivie integers (`usize`).
/// This is to make the comptations `pixel_size` more accurate.
pub struct Camera {
    /// The horizontal size (in pixels) of the canvas that the picture will be rendered to.
    hsize: f64,
    /// The vertical size (in pixels) of the canvas that the picture will be rendered to.
    vsize: f64,
    /// Angle that describes how much the camera can see. WHen the field of view is small, the wiew will be zoomed in,
    /// magnifying a smaller area of the scene.
    field_of_view: f64,
    /// Transform is a matrix describing how the world should be oriented relative to the camera.
    /// This is usually a view transformation.
    transform: Matrix<4>,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: f64, vsize: f64, field_of_view: f64, transform: Option<Matrix<4>>) -> Self {
        let (half_width, half_height, pixel_size) =
            Self::compute_pixel_size(hsize, vsize, field_of_view);
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: transform.unwrap_or_else(Matrix::<4>::identity),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub const fn hsize(&self) -> f64 {
        self.hsize
    }

    pub const fn vsize(&self) -> f64 {
        self.vsize
    }

    pub const fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub const fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub const fn half_width(&self) -> f64 {
        self.half_width
    }

    pub const fn half_height(&self) -> f64 {
        self.half_height
    }

    /// We know that the canvas is one unit away (from the camera), and the angle of the field of view.
    /// By cutting the field of view in half, we create a right triangle.
    /// The width of that half of the canvas can be computed by taking the tangent of
    /// half of the field of view (`half_view`):
    ///```
    ///let half_view = (field_of_view / 2.0).tan();
    ///```
    /// The aspect ratio is the ratio of the horizontal size of the canvas to its vertical size:
    ///```
    ///let aspect = hsize / vsize;
    ///```
    /// If the horizontal size if greater than or equal to the vertical size (`aspect` >= 1),
    /// then `half_view` is half the width of the canvas, and `half_view` / `aspect` is half the canvas's height.
    ///
    /// If the vertical size is greater than the horizontal size (`aspect` < 1),
    /// then `half_view` is instead half the height of the canvas, and half the canvas's width is `half_view` * `aspect`.
    ///
    /// The size of a single pixel on the canvas (`pixel_size`) is obtained by dividing the full width of the canvas (`half_width` * 2)
    /// by the horizontal size (in pixels) of the canvas (`hsize`).
    ///
    /// The function actually returns `(half_width, half_height, pixel_size)`.
    fn compute_pixel_size(hsize: f64, vsize: f64, field_of_view: f64) -> (f64, f64, f64) {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize;

        (half_width, half_height, pixel_size)
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    // TODO
    pub fn ray_for_pixel(pixel_x: f64, pixel_y: f64) -> Ray {
        Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0))
    }
}
