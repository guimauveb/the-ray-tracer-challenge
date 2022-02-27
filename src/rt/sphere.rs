use super::ray::Ray;
pub struct Sphere {
    id: u32,
    radii: f64,
}

pub trait Intersect<T> {
    fn intersect(&self, object: &T) -> Option<[f64; 2]>;
}

impl Sphere {
    pub fn new() -> Self {
        Sphere { id: 1, radii: 1.0 }
    }
}

impl Intersect<Ray> for Sphere {
    // If the ray intersects the sphere at two points P and P', we return [P, P']. If it intersects the sphere at one point P, we return [P, P]. Else we return None.
    fn intersect(&self, ray: &Ray) -> Option<[f64; 2]> {
        /* From https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection:
         * Geometric solution
         * - Get OC-> by computing the difference between O (ray origin) and C (sphere center)
         * - Compute the dot product of 'D' (the direction of 'O') and 'L' ('O' -> 'C' (sphere center))
         * - If the dot product is negative, no intersection (None)
         * - d is equal to (tca^2-L^2).sqrt()
         * - If d is negative or if d is > than the sphere radius, then there is no intersection.
         * - To find thc, we use the Pythagorean theorem again: thc = (radius^2 - d^2).sqrt()
         * - From there, we compute t0 (distance between 0 and P) and t1 (distance between 0 and P')
         *       - t0 = tca - thc
         *       - t1 = tca + thc
         * Analytic solution
         * TODO
         */
        Some([0.0, 0.0])
    }
}
