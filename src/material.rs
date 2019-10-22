use crate::color::*;
use crate::light::*;
use crate::texture::*;
use crate::transform::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub texture: Option<Texture>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            texture: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn specular(mut self, specular: f32) -> Self {
        self.specular = specular;
        self
    }

    pub fn shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn reflective(mut self, reflective: f32) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn transparency(mut self, transparency: f32) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn refractive_index(mut self, refractive_index: f32) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Computes the color of the surface at the given point.
    pub fn lighting(
        self,
        transform: Transform,
        light: Light,
        point: Tuple4,
        eyev: Tuple4,
        normalv: Tuple4,
        in_shadow: bool,
    ) -> Color {
        let base_color = match self.texture {
            Some(texture) => texture.evaluate(transform, point),
            None => self.color,
        };

        // Combine the surface color with the light's color/intensity.
        let effective_color = base_color * light.intensity;

        // Find the direction to the light source.
        let lightv = (light.position - point).normalize();

        // Compute and add the ambient contribution.
        let mut result = effective_color * self.ambient;

        // Skip the diffuse and specular components if the point is in shadow.
        if in_shadow {
            return result;
        }

        // light_dot_normal represents the cosine of the angle between the light
        // vector and the normal vector. A negative number means the light is on
        // the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal >= 0. {
            // Compute and add the diffuse contribution.
            result = result + effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye >= 0. {
                // Compute and add the specular contribution.
                let factor = reflect_dot_eye.powf(self.shininess);
                result = result + light.intensity * self.specular * factor;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn the_default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.reflective, 0.0);
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(0., 0., -1.);
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(
            0.,
            std::f32::consts::SQRT_2 / 2.,
            -std::f32::consts::SQRT_2 / 2.,
        );
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(0., 0., -1.);
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 10., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, false);
        assert_approx_eq!(result.r, 0.7364, 1e-5);
        assert_approx_eq!(result.g, 0.7364, 1e-5);
        assert_approx_eq!(result.b, 0.7364, 1e-5);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(
            0.,
            -std::f32::consts::SQRT_2 / 2.,
            -std::f32::consts::SQRT_2 / 2.,
        );
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 10., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, false);
        assert_approx_eq!(result.r, 1.6364, 1e-4);
        assert_approx_eq!(result.g, 1.6364, 1e-4);
        assert_approx_eq!(result.b, 1.6364, 1e-4);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(0., 0., -1.);
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 0., 10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, false);
        assert_approx_eq!(result.r, 0.1, 1e-5);
        assert_approx_eq!(result.g, 0.1, 1e-5);
        assert_approx_eq!(result.b, 0.1, 1e-5);
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = point3(0., 0., 0.);
        let eyev = vector3(0., 0., -1.);
        let normalv = vector3(0., 0., -1.);
        let light = Light::new(point3(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(Transform::new(), light, position, eyev, normalv, true);
        assert_approx_eq!(result.r, 0.1, 1e-5);
        assert_approx_eq!(result.g, 0.1, 1e-5);
        assert_approx_eq!(result.b, 0.1, 1e-5);
    }

    #[test]
    fn lighting_with_a_texture_applied() {
        let mut m = Material::new();
        m.texture = Some(Texture::stripe(Color::WHITE, Color::BLACK));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = vector3(0., 0., -1.);
        let normalv = vector3(0., 0., -1.0);
        let light = Light::new(point3(0., 0., -10.), Color::new(1., 1., 1.));
        let c1 = m.lighting(
            Transform::new(),
            light,
            point3(0.9, 0., 0.),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            Transform::new(),
            light,
            point3(1.1, 0., 0.),
            eyev,
            normalv,
            false,
        );
        assert_eq!(c1, Color::WHITE);
        assert_eq!(c2, Color::BLACK);
    }
}
