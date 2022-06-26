use {
    super::{material::Material, matrix::Matrix, shape::Shape},
    crate::tuple::{point::Point, vector::Vector},
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    //id: u32, // TODO ?
    origin: Point,
    transform: Matrix<4>,
    material: Material,
}

impl Sphere {
    pub const fn new(origin: Point, transform: Matrix<4>, material: Material) -> Self {
        Self {
            //id: 1,
            origin,
            transform,
            material,
        }
    }

    /// Creates a sphere with a given material. All other fields are set to default values.
    pub fn with_material(material: Material) -> Self {
        Self {
            material,
            ..Self::default()
        }
    }

    /// Creates a sphere with a given transform. All other fields are set to default values.
    pub fn with_transform(transform: Matrix<4>) -> Self {
        Self {
            transform,
            ..Self::default()
        }
    }

    pub const fn origin(&self) -> &Point {
        &self.origin
    }
}

impl Shape for Sphere {
    fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    /// Returns the normal vector at a specified point.
    fn normal_at(&self, point: &Point) -> Vector {
        // Convert the point from world space to object space
        let object_point = &self.transform.inverse().unwrap() * point;
        let object_normal = &object_point - &self.origin;
        // To keep the normal perpendicular to their surface, we multiply the object normal by the inverted then transposed object transform (matrix).
        let world_normal = self.transform.inverse().unwrap().transpose() * object_normal;

        world_normal.normalized()
    }
}

impl Default for Sphere {
    /// Creates a sphere centered at the origin and with a radius of 1.0.
    fn default() -> Self {
        Self {
            //id: 1,
            origin: Point::new(0.0, 0.0, 0.0),
            transform: Matrix::<4>::identity(),
            material: Material::default(),
        }
    }
}
