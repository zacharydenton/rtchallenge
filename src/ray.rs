use crate::matrix::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple4,
    pub direction: Tuple4,
}

/// Constructs a Ray with the given origin and direction.
pub fn ray(origin: Tuple4, direction: Tuple4) -> Ray {
    debug_assert!(origin.is_point());
    debug_assert!(direction.is_vector());
    Ray { origin, direction }
}

impl Ray {
    /// Computes the point at the given distance t along the ray.
    pub fn position(&self, t: f32) -> Tuple4 {
        self.origin + self.direction * t
    }

    /// Returns a new ray with the given transformation matrix applied to origin
    /// and direction.
    pub fn transform(&self, matrix: Matrix4) -> Ray {
        ray(matrix * self.origin, matrix * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point3(1., 2., 3.);
        let direction = vector3(4., 5., 6.);
        let r = ray(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = ray(point3(2., 3., 4.), vector3(1., 0., 0.));
        assert_eq!(r.position(0.0), point3(2., 3., 4.,));
        assert_eq!(r.position(1.0), point3(3., 3., 4.,));
        assert_eq!(r.position(-1.0), point3(1., 3., 4.,));
        assert_eq!(r.position(2.5), point3(4.5, 3., 4.,));
    }

    #[test]
    fn translating_a_ray() {
        let r = ray(point3(1., 2., 3.), vector3(0., 1., 0.));
        let m = Transform::new().translate(3., 4., 5.).local_to_world;
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point3(4., 6., 8.));
        assert_eq!(r2.direction, vector3(0., 1., 0.,));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point3(1., 2., 3.), vector3(0., 1., 0.));
        let m = Transform::new().scale(2., 3., 4.).local_to_world;
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point3(2., 6., 12.));
        assert_eq!(r2.direction, vector3(0., 3., 0.,));
    }
}
