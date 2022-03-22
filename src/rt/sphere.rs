use {
    super::{intersection::Intersection, ray::Ray},
    crate::{
        primitive::{point::Point, tuple::Tuple},
        rt::intersect::Intersect,
    },
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    id: u32, //?
    origin: Point,
    radii: f64, //?
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            id: 1,
            origin: Point::new(0.0, 0.0, 0.0), // TODO
            radii: 1.0,
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
}

impl Intersect<Ray> for Sphere {
    // If the ray intersects the sphere at two points P and P', we return [P, P']. If it intersects the sphere at one point P, we return [P, P]. Else we return None.
    fn intersect(&self, ray: &Ray) -> Option<[Intersection<'_, Sphere>; 2]> {
        /* From https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection:
         1. Geometric solution
            - Get OC-> by computing the difference between O (ray origin) and C (sphere center)
            - Compute the dot product of 'D' (the direction of 'O') and 'L' ('O' -> 'C' (sphere center))
            - If the dot product is negative, no intersection (None)
            - d is equal to (tca^2-L^2).sqrt()
            - If d is negative or if d is > than the sphere radius, then there is no intersection.
            - To find thc, we use the Pythagorean theorem again: thc = (radius^2 - d^2).sqrt()
            - From there, we compute t0 (distance between 0 and P) and t1 (distance between 0 and P')
                  - t0 = tca - thc
                  - t1 = tca + thc
         2. Analytic solution
            - We know that the equation of a sphere can be written as the following:
                  - x^2 + y^2 + z^2 = R^2
                   - P^2 = R^2 (P being a (x, y, z) point)
              - And that the equation of the ray is the following:
                  - O + Dt
              - So to find the intersection of the sphere and the ray, we need to find the values for which both functions yield the same result, which can be
              written as the following:
                 - |O + Dt|^2 - R^2 = 0 (if the sphere is centered at the origin)
                 - |O + Dt - C|^2 - R^2 = 0 (C being the center of the sphere)
            - which we can develop to arrive at a quadratic equation which we can solve:
              - D^2t^2 + 2D(O-C) + |O - C|^2 - R^2
               where
                  a = D^2,
                  b = 2D(O-C),
                  c = |O-C|^2 - R^2
        */
        let sphere_to_ray = ray.origin() - self.origin();
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else if discriminant == 0.0 {
            let t0 = -b / (2.0 * a);
            Some([Intersection::new(t0, &self), Intersection::new(t0, &self)])
        } else {
            let (t0, t1) = (
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            );
            Some([Intersection::new(t0, &self), Intersection::new(t1, &self)])
        }
    }
}
