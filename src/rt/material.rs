use {
    super::{
        color::{Color, BLACK, WHITE},
        object::Object,
        pattern::Pattern,
        point_light::PointLight,
    },
    crate::{
        approx_eq::ApproxEq,
        tuple::{point::Point, vector::Vector},
    },
};

#[derive(Debug)]
pub struct Material {
    color: Color,
    pattern: Option<Pattern>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
}

impl PartialEq for Material {
    fn eq(&self, rhs: &Self) -> bool {
        // Since patterns are actually functions,
        // we cannot compare them.
        if self.pattern().is_some() {
            false
        } else {
            self.color == rhs.color
                && self.ambient.approx_eq(rhs.ambient)
                && self.diffuse.approx_eq(rhs.diffuse)
                && self.specular.approx_eq(rhs.specular)
                && self.shininess.approx_eq(rhs.shininess)
                && self.reflective.approx_eq(rhs.reflective)
                && self.transparency.approx_eq(rhs.transparency)
                && self.refractive_index.approx_eq(rhs.refractive_index)
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        if self.pattern().is_some() {
            panic!("Cannot duplicate a Material containing a Pattern.");
        } else {
            Self {
                color: self.color,
                pattern: None,
                ambient: self.ambient,
                diffuse: self.diffuse,
                specular: self.specular,
                shininess: self.shininess,
                reflective: self.reflective,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            }
        }
    }
}

impl Default for Material {
    /// Creates a Material with the following values:
    /// ```
    /// let material = Material {
    ///     color: Color { 1.0, 1.0, 1.0 },
    ///     pattern: None,
    ///     ambient: 0.1,
    ///     diffuse: 0.9,
    ///     specular: 0.9,
    ///     shininess: 200.0,
    ///     reflective: 0.0,
    ///     transparency: 0.0,
    ///     refractive_index: 1.0,
    /// };
    /// ```
    fn default() -> Self {
        Self {
            color: WHITE,
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl Material {
    pub const fn new(
        color: Color,
        pattern: Option<Pattern>,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Self {
        Self {
            color,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
        }
    }

    pub const fn color(&self) -> &Color {
        &self.color
    }

    pub const fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }

    pub const fn ambient(&self) -> f64 {
        self.ambient
    }

    pub const fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub const fn specular(&self) -> f64 {
        self.specular
    }

    pub const fn shininess(&self) -> f64 {
        self.shininess
    }

    pub const fn reflective(&self) -> f64 {
        self.reflective
    }

    pub const fn transparency(&self) -> f64 {
        self.transparency
    }

    pub const fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular;
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess;
    }

    pub fn set_reflective(&mut self, reflective: f64) {
        self.reflective = reflective;
    }

    pub fn set_transparency(&mut self, transparency: f64) {
        self.transparency = transparency;
    }

    pub fn set_refractive_index(&mut self, refractive_index: f64) {
        self.refractive_index = refractive_index;
    }

    /// Returns the color of the material at a specified point from a specified view point.
    pub fn lighting(
        &self,
        object: &Object,
        light: &PointLight,
        point: &Point,
        eye: &Vector,
        normal: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = self
            .pattern
            .as_ref()
            .map_or_else(|| self.color, |p| p.at_object(object, point));
        // Combine the surface color with the light intensity
        let effective_color = &color * light.intensity();
        // Find the direction to the light source (point -> light source)
        let point_to_light = (light.position() - point).normalized();
        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;
        /* light_dot_normal represents the cosine of the angle between the
         * light vector and the normal vector. A negative number means
         * the light is on the other side of the surface. */
        let light_dot_normal = point_to_light.dot(normal);
        let (diffuse, specular) = if in_shadow || light_dot_normal < 0.0 {
            (BLACK, BLACK)
        } else {
            // Compute the diffuse contribution
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            /* reflect_dot_eye represents the cosine of the angle between the
             * reflection vector and the eye vector. A negative number means the
             * light reflects away from the eye. */
            let reflect = -point_to_light.reflect(normal);
            let reflect_dot_eye = reflect.dot(eye);
            let specular = if reflect_dot_eye <= 0.0 {
                BLACK
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
