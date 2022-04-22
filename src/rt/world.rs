use crate::{
    primitive::{point::Point, tuple::Tuple},
    rt::{
        color::Color,
        computation::Computation,
        material::Material,
        matrix::{Matrix, Scaling},
        object::Object,
        point_light::PointLight,
        sphere::Sphere,
    },
};

#[derive(PartialEq, Debug)]
pub struct World {
    objects: Option<Vec<Object>>,
    light: Option<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
        let s1 = Sphere::with_material(material);

        let transform = Matrix::<4>::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::with_transform(transform);

        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        Self {
            objects: Some(vec![Object::Sphere(s1), Object::Sphere(s2)]),
            light: Some(light),
        }
    }
}

impl World {
    /// Creates an empty world.
    pub fn empty() -> Self {
        Self {
            objects: None,
            light: None,
        }
    }

    pub fn new(objects: Option<Vec<Object>>, light: Option<PointLight>) -> Self {
        Self { objects, light }
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

    pub fn objects(&self) -> Option<&[Object]> {
        self.objects.as_deref()
    }

    pub fn light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }

    pub fn shade_hit(&self, computations: &Computation) {}
}
