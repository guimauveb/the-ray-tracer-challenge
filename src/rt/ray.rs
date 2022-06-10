use {
    super::{
        intersections::Intersections,
        matrix::{Matrix, Transform},
        object::Object,
        plane::Plane,
        shape::Shape,
        world::World,
    },
    crate::{
        approx_eq::ApproxEq,
        float::epsilon::EPSILON,
        rt::sphere::Sphere,
        tuple::{point::Point, vector::Vector},
    },
};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub const fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub const fn origin(&self) -> &Point {
        &self.origin
    }

    pub const fn direction(&self) -> &Vector {
        &self.direction
    }

    /// Computes the point at the given distance along the ray.
    pub fn position(&self, distance: f32) -> Point {
        &self.origin + &self.direction * distance
    }
}

impl Transform for Ray {
    fn transform(&self, m: &Matrix<4>) -> Self {
        Self {
            origin: m * &self.origin,
            direction: m * &self.direction,
        }
    }
}

/// Describes how a ray intersects with one or multiple objects.
/// `O` is the object (most likely an Object or a World (composed of many objects)) being intersected.
/// `I` is the type of the intersection returned (could be of type `[f32; 2]` or `Intersections` for instance).
pub trait Intersect<'object, O, I> {
    /// Before computing the intersections, the ray
    /// must first be converted into object space by
    /// multiplying it by the inverse of the object
    /// transformation matrix.
    fn intersect(&self, object: &'object O) -> Option<I>;
}

impl<'object> Intersect<'object, Sphere, [f32; 2]> for Ray {
    /// If the ray intersects the sphere at two points P and P', we return [P, P']. If it intersects the sphere at one point P, we return [P, P]. Else we return None.
    ///  From <https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection>:
    ///  1. Geometric solution
    ///     - Get OC-> by computing the difference between O (ray origin) and C (sphere center)
    ///     - Compute the dot product of 'D' (the direction of 'O') and 'L' ('O' -> 'C' (sphere center))
    ///     - If the dot product is negative, no intersection (None)
    ///     - d is equal to (tca^2-L^2).sqrt()
    ///     - If d is negative or if d is > than the sphere radius, then there is no intersection.
    ///     - To find thc, we use the Pythagorean theorem again: thc = (radius^2 - d^2).sqrt()
    ///     - From there, we compute t0 (distance between 0 and P) and t1 (distance between 0 and P')
    ///           - t0 = tca - thc
    ///           - t1 = tca + thc
    ///  2. Analytic solution
    ///     - We know that the equation of a sphere can be written as the following:
    ///           - x^2 + y^2 + z^2 = R^2
    ///            - P^2 = R^2 (P being a (x, y, z) point)
    ///       - And that the equation of the ray is the following:
    ///           - O + Dt
    ///       - So to find the intersection of the sphere and the ray, we need to find the values for which both functions yield the same result, which can be
    ///       written as the following:
    ///          - |O + Dt|^2 - R^2 = 0 (if the sphere is centered at the origin)
    ///          - |O + Dt - C|^2 - R^2 = 0 (C being the center of the sphere)
    ///     - which we can develop to arrive at a quadratic equation which we can solve:
    ///       - D^2t^2 + 2D(O-C) + |O - C|^2 - R^2
    ///        where
    ///           a = D^2,
    ///           b = 2D(O-C),
    ///           c = |O-C|^2 - R^2
    fn intersect(&self, sphere: &Sphere) -> Option<[f32; 2]> {
        let transformed_ray = self.transform(&sphere.get_transform().inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin() - sphere.origin();
        let a = transformed_ray.direction().dot(transformed_ray.direction());
        let b = 2.0 * transformed_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else if discriminant.approx_eq(0.0) {
            let t0 = -b / (2.0 * a);
            Some([t0, t0])
        } else {
            let (t0, t1) = (
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            );
            Some([t0, t1])
        }
    }
}

impl<'object> Intersect<'object, Plane, [f32; 2]> for Ray {
    fn intersect(&self, plane: &Plane) -> Option<[f32; 2]> {
        let transformed_ray = self.transform(&plane.get_transform().inverse().unwrap());
        // The plane is defined in xz, it has no slope in y at all.
        // Thus, if a ray's direction vector also has no slope in y
        // It's parallel to the plane. In practice we'll treat any
        // number smaller than EPSILON as 0.0.
        if transformed_ray.direction.y().abs() < EPSILON {
            None
        } else {
            let t = -transformed_ray.origin.y() / transformed_ray.direction.y();
            Some([t, t])
        }
    }
}

impl<'object> Intersect<'object, Object, Intersections<'object>> for Ray {
    /// Returns a list of intersections with the object.
    fn intersect(&self, object: &'object Object) -> Option<Intersections<'object>> {
        match object {
            Object::Sphere(sphere) => self.intersect(sphere).map(|xs| (xs, object).into()),
            Object::Plane(plane) => self.intersect(plane).map(|xs| (xs, object).into()),
        }
    }
}
/// Before computing the normal at some point,
/// all shapes must first convert the point to
/// object space by multiplying it by the inverse
/// of the shape's transformation matrix.
impl<'objects> Intersect<'objects, World, Intersections<'objects>> for Ray {
    /// Returns a list of intersections with the objects composing the world.
    fn intersect(&self, world: &'objects World) -> Option<Intersections<'objects>> {
        world.objects().map(|objects| {
            Intersections::new(
                objects
                    .iter()
                    .filter_map(|object| self.intersect(object))
                    .flatten()
                    .collect(),
            )
        })
    }
}
