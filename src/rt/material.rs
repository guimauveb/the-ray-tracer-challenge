use {
    super::{color::Color, lighting::Lighting, point_light::PointLight},
    crate::primitive::{point::Point, vector::Vector},
};

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Lighting for Material {
    fn lighting(&self, light: &PointLight, point: &Point, eye: &Vector, normal: &Vector) -> Color {
        // Combine the surface color with the light intensity
        let effective_color = &self.color * light.intensity();
        // Find the direction of the light source (point -> light source)
        let lightv = (light.position() - point).normalize();
        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;
        /* light_dot_normal represents the cosine of the angle between the
         * light vector and the normal vector. A negative number means
         * the light is on the other side of the surface. */
        let light_dot_normal = lightv.dot(normal);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0))
        } else {
            // Compute the diffuse contribution
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            /* reflect_dot_eye represents the cosine of the angle between the
             * reflection vector and the eye vector. A negative number means the
             * light reflects away from the eye. */
            let reflect = -lightv.reflect(normal);
            let reflect_dot_eye = reflect.dot(eye);
            let specular = if reflect_dot_eye <= 0.0 {
                Color::new(0.0, 0.0, 0.0)
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess); // NOTE - Use a factor power of 2 for faster computation?
                light.intensity() * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess
    }
}
