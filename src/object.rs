use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere {},
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix4,
    pub material: Material,
}

/// Constructs a unit sphere centered at the origin (0, 0, 0).
pub fn sphere() -> Object {
    Object {
        shape: Shape::Sphere {},
        transform: I4,
        material: material(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Object,
}

type Intersections<'a> = Vec<Intersection<'a>>;

impl Object {
    /// Returns the collection of Intersections where the ray intersects the object.
    pub fn intersect(&self, ray: Ray) -> Intersections {
        match self.shape {
            Shape::Sphere {} => {
                // Instead of transforming the sphere, apply the inverse
                // transformation to the ray.
                let ray = ray.transform(self.transform.inverse());

                // The vector from the sphere's center, to the ray origin
                // (remember: the sphere is centered at the world origin)
                let sphere_to_ray = ray.origin - point3(0., 0., 0.);

                let a = ray.direction.dot(ray.direction);
                let b = 2. * ray.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

                let discriminant = b * b - 4. * a * c;

                if discriminant < 0. {
                    vec![]
                } else {
                    let t1 = (-b - discriminant.sqrt()) / (2. * a);
                    let t2 = (-b + discriminant.sqrt()) / (2. * a);
                    vec![
                        Intersection {
                            t: t1,
                            object: self,
                        },
                        Intersection {
                            t: t2,
                            object: self,
                        },
                    ]
                }
            }
        }
    }

    /// Returns the surface normal at the given point.
    pub fn normal(&self, point: Tuple4) -> Tuple4 {
        match self.shape {
            Shape::Sphere {} => {
                let inverse_transform = self.transform.inverse();
                let object_point = inverse_transform * point;
                let object_normal = object_point - point3(0., 0., 0.);
                let mut world_normal = inverse_transform.transpose() * object_normal;
                world_normal.w = 0.;
                world_normal.normalize()
            }
        }
    }
}

pub fn hit(intersections: Intersections) -> Option<Intersection> {
    intersections
        .iter()
        .filter(|i| i.t >= 0.)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .map(|&i| i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;
    use assert_approx_eq::assert_approx_eq;

    fn intersection<'a>(t: f32, object: &'a Object) -> Intersection<'a> {
        Intersection { t, object }
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, vec![intersection(4.0, &s), intersection(6.0, &s)]);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point3(0., 1., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, vec![intersection(5.0, &s), intersection(5.0, &s)]);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point3(0., 2., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, vec![intersection(-1.0, &s), intersection(1.0, &s)]);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, vec![intersection(-6.0, &s), intersection(-4.0, &s)]);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = sphere();
        assert_eq!(s.transform, I4);
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = sphere();
        let t = translate(2., 3., 4.);
        s.transform = t;
        assert_eq!(s.transform, t);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = sphere();
        assert_eq!(s.material, material());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = sphere();
        let mut m = material();
        m.ambient = 1.;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut s = sphere();
        s.transform = scale(2., 2., 2.);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut s = sphere();
        s.transform = translate(5., 0., 0.);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = s.normal(point3(1., 0., 0.));
        assert_eq!(n, vector3(1., 0., 0.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = s.normal(point3(0., 1., 0.));
        assert_eq!(n, vector3(0., 1., 0.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = s.normal(point3(0., 0., 1.));
        assert_eq!(n, vector3(0., 0., 1.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = sphere();
        let root3over3 = (3 as f32).sqrt() / 3.;
        let n = s.normal(point3(root3over3, root3over3, root3over3));
        assert_approx_eq!(n.x, root3over3);
        assert_approx_eq!(n.y, root3over3);
        assert_approx_eq!(n.z, root3over3);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = sphere();
        let root3over3 = (3 as f32).sqrt() / 3.;
        let n = s.normal(point3(root3over3, root3over3, root3over3));
        let normalized = n.normalize();
        assert_approx_eq!(n.x, normalized.x);
        assert_approx_eq!(n.y, normalized.y);
        assert_approx_eq!(n.z, normalized.z);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = sphere();
        s.transform = translate(0., 1., 0.);
        let n = s.normal(point3(0., 1.70711, -0.70711));
        assert_approx_eq!(n.x, 0., 1e-5);
        assert_approx_eq!(n.y, 0.70711, 1e-5);
        assert_approx_eq!(n.z, -0.70711, 1e-5);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = sphere();
        let m = scale(1., 0.5, 1.) * rotate_z(std::f32::consts::PI / 5.);
        s.transform = m;
        let n = s.normal(point3(
            0.,
            2. * std::f32::consts::FRAC_1_SQRT_2,
            -2. * std::f32::consts::FRAC_1_SQRT_2,
        ));
        assert_approx_eq!(n.x, 0., 1e-5);
        assert_approx_eq!(n.y, 0.97014, 1e-5);
        assert_approx_eq!(n.z, -0.24254, 1e-5);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1., &s);
        let i2 = intersection(2., &s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1., &s);
        let i2 = intersection(1., &s);
        let xs = vec![i2, i1];
        let i = hit(xs);
        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2., &s);
        let i2 = intersection(-1., &s);
        let xs = vec![i2, i1];
        let i = hit(xs);
        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5., &s);
        let i2 = intersection(7., &s);
        let i3 = intersection(-3., &s);
        let i4 = intersection(2., &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs);
        assert_eq!(i, Some(i4));
    }
}
