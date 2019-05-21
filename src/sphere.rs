use crate::ray::*;
use crate::tuple::*;

pub struct Sphere {}

/// Constructs a unit sphere centered at the origin (0, 0, 0).
pub fn sphere() -> Sphere {
    Sphere {}
}

impl Sphere {
    /// Returns the collection of t values where the ray intersects the sphere.
    ///
    /// In the case where two unique intersections are found, the lesser value
    /// of t will come first.
    pub fn intersect(&self, ray: Ray) -> Option<(f32, f32)> {
        // The vector from the sphere's center, to the ray origin
        // (remember: the sphere is centered at the world origin)
        let sphere_to_ray = ray.origin - point3(0., 0., 0.);

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            Some((t1, t2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((4.0, 6.0)));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point3(0., 1., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((5.0, 5.0)));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point3(0., 2., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(None, xs);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((-1.0, 1.0)));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((-6.0, -4.0)));
    }
}
