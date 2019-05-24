use crate::color::*;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
enum LightKind {
    PointLight,
}

#[derive(Debug)]
pub struct Light {
    kind: LightKind,
    pub position: Tuple4,
    pub intensity: Color,
}

pub fn point_light(position: Tuple4, intensity: Color) -> Light {
    Light {
        kind: LightKind::PointLight,
        position,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = color(1., 1., 1.);
        let position = point3(0., 0., 0.);
        let light = point_light(position, intensity);
        assert_eq!(light.kind, LightKind::PointLight);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
