use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub enum Object {
    Sphere {},
}

/// Constructs a unit sphere centered at the origin (0, 0, 0).
pub fn sphere() -> Object {
    Object::Sphere {}
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Object,
}

impl Object {
    /// Returns the collection of Intersections where the ray intersects the object.
    pub fn intersect(&self, ray: Ray) -> Option<(Intersection, Intersection)> {
        match *self {
            Object::Sphere {} => {
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
                    Some((
                        Intersection {
                            t: t1,
                            object: self,
                        },
                        Intersection {
                            t: t2,
                            object: self,
                        },
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intersection<'a>(t: f32, object: &'a Object) -> Intersection<'a> {
        Intersection { t, object }
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((intersection(4.0, &s), intersection(6.0, &s))));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point3(0., 1., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((intersection(5.0, &s), intersection(5.0, &s))));
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
        assert_eq!(xs, Some((intersection(-1.0, &s), intersection(1.0, &s))));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((intersection(-6.0, &s), intersection(-4.0, &s))));
    }
}
