use crate::{
    rt::{
        color::Color,
        computation::Computation,
        material::Material,
        matrix::Matrix,
        object::Object,
        point_light::PointLight,
        ray::{Intersect, Ray},
        shape::Shape,
        sphere::Sphere,
    },
    tuple::point::Point,
};

/// Maximum number of times `reflected_color` can be called before stopping
/// the recursion and returning `Color::black()` by default.
/// The puprpose of this limit is to avoid infinite recursion in the case
/// where two surfaces reflect each other.
pub const MAX_REFLECTION_DEPTH: u8 = 6;

#[derive(PartialEq, Debug)]
pub struct World {
    objects: Option<Vec<Object>>,
    light: Option<PointLight>,
}

impl World {
    const fn default_light() -> PointLight {
        PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white())
    }

    /// Creates a new world.
    pub const fn new(objects: Option<Vec<Object>>, light: Option<PointLight>) -> Self {
        Self { objects, light }
    }

    /// Creates an empty world.
    pub const fn empty() -> Self {
        Self {
            objects: None,
            light: None,
        }
    }

    /// Creates a default world with a specified light.
    pub fn with_light(light: Option<PointLight>) -> Self {
        Self {
            light,
            ..Self::default()
        }
    }

    /// Creates a default world with a specified set of objects.
    pub const fn with_objects(objects: Option<Vec<Object>>) -> Self {
        Self {
            objects,
            light: Some(Self::default_light()),
        }
    }

    pub fn objects(&self) -> Option<&[Object]> {
        self.objects.as_deref()
    }

    pub fn objects_mut(&mut self) -> Option<&mut [Object]> {
        self.objects.as_deref_mut()
    }

    pub const fn light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn add_object(&mut self, object: Object) {
        if let Some(objects) = &mut self.objects {
            objects.push(object);
        } else {
            self.objects = Some(vec![object]);
        }
    }

    /// Intersects the world with the given ray and returns the color at the resulting intersection.
    pub fn color_at(&self, ray: &Ray, remaining_calls: u8) -> Color {
        if let Some(intersections) = ray.intersect(self) {
            if let Some(hit) = intersections.hit() {
                return self.shade_hit(&hit.prepare_computations(ray), remaining_calls);
            }
        }
        Color::black()
    }

    pub fn shade_hit(&self, computations: &Computation, remaining_calls: u8) -> Color {
        let surface = computations.intersection().object().material().lighting(
            computations.intersection().object(),
            self.light.as_ref().expect("World should have a light!"),
            computations.over_point(),
            computations.eye_vector(),
            computations.normal_vector(),
            self.is_shadowed(computations.over_point()),
        );
        let reflected = self.reflected_color(computations, remaining_calls);

        surface + reflected
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        let point_to_light = self
            .light
            .as_ref()
            .expect("This world has no light!")
            .position()
            - point;
        let distance = point_to_light.magnitude();
        let direction = point_to_light.normalized();
        let ray = Ray::new(point.clone(), direction);
        let intersections = ray.intersect(self);
        if let Some(intersections) = intersections {
            let hit = intersections.hit();
            if let Some(hit) = hit {
                return hit.t() < distance;
            }
            return false;
        }
        false
    }

    pub fn reflected_color(&self, computations: &Computation, remaining_calls: u8) -> Color {
        if remaining_calls == 0
            || computations.intersection().object().material().reflective() == 0.0
        {
            Color::black()
        } else {
            let reflect_ray = Ray::new(
                computations.over_point().clone(),
                computations.reflect_vector().clone(),
            );
            let color = self.color_at(&reflect_ray, remaining_calls - 1);
            color * computations.intersection().object().material().reflective()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let material = Material::new(Color::new(0.8, 1.0, 0.6), None, 0.1, 0.7, 0.2, 200.0, 0.0);
        let s1 = Sphere::with_material(material);

        let transform = Matrix::<4>::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::with_transform(transform);

        Self {
            objects: Some(vec![s1.into(), s2.into()]),
            light: Some(Self::default_light()),
        }
    }
}
