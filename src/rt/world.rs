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

    pub fn shade_hit(&self, computations: &Computation) -> Color {
        computations
            .intersection()
            .object()
            .get_material()
            .lighting(
                computations.intersection().object(),
                self.light.as_ref().expect("World should have a light!"),
                computations.over_point(),
                computations.eye_vector(),
                computations.normal_vector(),
                self.is_shadowed(computations.over_point()),
            )
    }

    /// Intersects the world with the given ray and returns the color at the resulting intersection.
    pub fn color_at(&self, ray: &Ray) -> Color {
        if let Some(intersections) = ray.intersect(self) {
            if let Some(hit) = intersections.hit() {
                return self.shade_hit(&hit.prepare_computations(ray));
            }
        }
        Color::black()
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
}

impl Default for World {
    fn default() -> Self {
        let material = Material::new(Color::new(0.8, 1.0, 0.6), None, 0.1, 0.7, 0.2, 200.0);
        let s1 = Sphere::with_material(material);

        let transform = Matrix::<4>::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::with_transform(transform);

        Self {
            objects: Some(vec![s1.into(), s2.into()]),
            light: Some(Self::default_light()),
        }
    }
}
