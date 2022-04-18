use {
    super::{
        intersect::Intersect, intersection::Intersection, intersections::Intersections,
        matrix::Matrix, object::Object, transform::Transform, world::World,
    },
    crate::{
        primitive::{point::Point, vector::Vector},
        rt::sphere::Sphere,
    },
};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

pub trait Position {
    fn position(&self, distance: f64) -> Point;
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

impl Position for Ray {
    fn position(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}

impl<'a> Intersect<'a, Sphere> for Ray {
    // If the ray intersects the sphere at two points P and P', we return [P, P']. If it intersects the sphere at one point P, we return [P, P]. Else we return None.
    fn intersect(&self, sphere: &'a Sphere) -> Option<Intersections<'a>> {
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

        // NOTE - Use a mutable reference to self instead?
        let transformed_ray = self.transform(&sphere.transform().inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin() - sphere.origin();
        let a = transformed_ray.direction().dot(transformed_ray.direction());
        let b = 2.0 * transformed_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else if discriminant == 0.0 {
            let t0 = -b / (2.0 * a);
            Some(Intersections::new(vec![
                Intersection::Sphere(t0, sphere),
                Intersection::Sphere(t0, sphere),
            ]))
        } else {
            let (t0, t1) = (
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            );
            Some(Intersections::new(vec![
                Intersection::Sphere(t0, sphere),
                Intersection::Sphere(t1, sphere),
            ]))
        }
    }
}

impl<'a> Intersect<'a, World> for Ray {
    fn intersect(&self, world: &'a World) -> Option<Intersections<'a>> {
        if let Some(objects) = world.objects() {
            // Reserve memory for at least (number of objects * 2), since each objects can at most be intersected at two points (at least for now).
            let mut intersections: Intersections<'a> =
                Intersections::with_capacity(objects.len() * 2);
            for object in objects {
                if let Object::Sphere(sphere) = object {
                    let sphere_xs = self.intersect(sphere);
                    if let Some(xs) = sphere_xs {
                        intersections.extend(xs);
                    }
                }
            }

            if !intersections.is_empty() {
                Some(intersections)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Transform for Ray {
    fn transform(&self, m: &Matrix<4_usize>) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}
