use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Plane {},
    Sphere {},
    TestShape {},
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix4,
    pub material: Material,
}

/// Constructs a plane centered at the origin (0, 0, 0).
pub fn plane() -> Object {
    Object {
        shape: Shape::Plane {},
        transform: I4,
        material: material(),
    }
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
    pub point: Option<Tuple4>,
    pub over_point: Option<Tuple4>,
    pub eyev: Option<Tuple4>,
    pub normalv: Option<Tuple4>,
    pub reflectv: Option<Tuple4>,
    pub inside: Option<bool>,
}

impl Intersection<'_> {
    #[inline]
    fn prepare(&mut self, ray: &Ray, inverse_transform: Matrix4) -> Self {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_inv(point, inverse_transform);

        self.point = Some(point);
        self.eyev = Some(eyev);

        if normalv.dot(eyev) < 0. {
            normalv = -normalv;
            self.inside = Some(true);
        } else {
            self.inside = Some(false);
        }

        self.normalv = Some(normalv);
        self.reflectv = Some(ray.direction.reflect(normalv));
        self.over_point = Some(point + normalv * 1e-2);

        *self
    }
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

impl Object {
    /// Returns the collection of Intersections where the ray intersects the object.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        // Instead of transforming the object, apply the inverse transformation to the ray.
        let inverse_transform = self.transform.inverse();
        let transformed_ray = ray.transform(inverse_transform);

        match self.shape {
            Shape::Plane {} => {
                if transformed_ray.direction.y.abs() < 1e-2 {
                    return vec![];
                }

                let t = -transformed_ray.origin.y / transformed_ray.direction.y;
                vec![Intersection {
                    t: t,
                    object: self,
                    point: None,
                    over_point: None,
                    eyev: None,
                    normalv: None,
                    reflectv: None,
                    inside: None,
                }
                .prepare(ray, inverse_transform)]
            }
            Shape::Sphere {} => {
                // The vector from the sphere's center, to the ray origin
                // (remember: the sphere is centered at the world origin)
                let sphere_to_ray = transformed_ray.origin - point3(0., 0., 0.);

                let a = transformed_ray.direction.dot(transformed_ray.direction);
                let b = 2. * transformed_ray.direction.dot(sphere_to_ray);
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
                            point: None,
                            over_point: None,
                            eyev: None,
                            normalv: None,
                            reflectv: None,
                            inside: None,
                        }
                        .prepare(ray, inverse_transform),
                        Intersection {
                            t: t2,
                            object: self,
                            point: None,
                            over_point: None,
                            eyev: None,
                            normalv: None,
                            reflectv: None,
                            inside: None,
                        }
                        .prepare(ray, inverse_transform),
                    ]
                }
            }
            Shape::TestShape {} => vec![],
        }
    }

    /// Returns the surface normal at the given point.
    pub fn normal(&self, point: Tuple4) -> Tuple4 {
        self.normal_inv(point, self.transform.inverse())
    }

    /// Returns the surface normal at the given point (with precalculated
    /// inverse transform).
    fn normal_inv(&self, point: Tuple4, inverse_transform: Matrix4) -> Tuple4 {
        let object_normal = match self.shape {
            Shape::Plane {} => vector3(0., 1., 0.),
            _ => {
                let object_point = inverse_transform * point;
                object_point - point3(0., 0., 0.)
            }
        };

        let mut world_normal = inverse_transform.transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
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
    use test::Bencher;

    fn intersection(t: f32, object: &Object) -> Intersection {
        Intersection {
            t,
            object,
            point: None,
            over_point: None,
            eyev: None,
            normalv: None,
            reflectv: None,
            inside: None,
        }
    }

    fn test_object() -> Object {
        Object {
            shape: Shape::TestShape {},
            transform: I4,
            material: material(),
        }
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point3(0., 1., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].t, 5.0);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point3(0., 2., -5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].t, 1.0);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].t, -4.0);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn an_objects_default_transformation() {
        let o = test_object();
        assert_eq!(o.transform, I4);
    }

    #[test]
    fn changing_an_objects_transformation() {
        let mut o = test_object();
        let t = translate(2., 3., 4.);
        o.transform = t;
        assert_eq!(o.transform, t);
    }

    #[test]
    fn an_object_has_a_default_material() {
        let s = test_object();
        assert_eq!(s.material, material());
    }

    #[test]
    fn an_object_may_be_assigned_a_material() {
        let mut s = test_object();
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
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut s = sphere();
        s.transform = translate(5., 0., 0.);
        let xs = s.intersect(&r);
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
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = plane();
        let n1 = p.normal(point3(0., 0., 0.));
        let n2 = p.normal(point3(10., 0., -10.));
        let n3 = p.normal(point3(-5., 0., 150.));
        assert_eq!(n1, vector3(0., 1., 0.,));
        assert_eq!(n2, vector3(0., 1., 0.,));
        assert_eq!(n3, vector3(0., 1., 0.,));
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

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let shape = sphere();
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[0].object, &shape);
        assert_eq!(xs[0].point, Some(point3(0., 0., -1.)));
        assert_eq!(xs[0].eyev, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[0].normalv, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[0].inside, Some(false));
        assert_eq!(xs[1].t, 6.);
        assert_eq!(xs[1].object, &shape);
        assert_eq!(xs[1].point, Some(point3(0., 0., 1.)));
        assert_eq!(xs[1].eyev, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[1].normalv, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[1].inside, Some(true));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let shape = sphere();
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[0].object, &shape);
        assert_eq!(xs[0].point, Some(point3(0., 0., -1.)));
        assert_eq!(xs[0].eyev, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[0].normalv, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[0].inside, Some(false));
        assert_eq!(xs[1].t, 1.);
        assert_eq!(xs[1].object, &shape);
        assert_eq!(xs[1].point, Some(point3(0., 0., 1.)));
        assert_eq!(xs[1].eyev, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[1].normalv, Some(vector3(0., 0., -1.)));
        assert_eq!(xs[1].inside, Some(true));
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = plane();
        let r = ray(
            point3(0., 1., -1.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );
        let mut i = intersection(std::f32::consts::SQRT_2, &shape);
        i.prepare(&r, shape.transform.inverse());
        if let Some(reflectv) = i.reflectv {
            assert_approx_eq!(reflectv.x, 0.0);
            assert_approx_eq!(reflectv.y, std::f32::consts::SQRT_2 * 0.5);
            assert_approx_eq!(reflectv.z, std::f32::consts::SQRT_2 * 0.5);
        } else {
            panic!();
        }
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut shape = sphere();
        shape.transform = translate(0., 0., 1.);

        let xs = shape.intersect(&r);
        let i = xs.iter().find(|x| x.t == 5.).unwrap();

        assert!(i.over_point.unwrap().z < -1e-2 / 2.);
        assert!(i.point.unwrap().z > i.over_point.unwrap().z);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = plane();
        let r = ray(point3(0., 10., 0.), vector3(0., 0., 1.));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = plane();
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = plane();
        let r = ray(point3(0., 1., 0.), vector3(0., -1., 0.));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = plane();
        let r = ray(point3(0., -1., 0.), vector3(0., 1., 0.));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, &p);
    }

    #[bench]
    fn bench_intersecting_a_scaled_sphere_with_a_ray(bencher: &mut Bencher) {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut s = sphere();
        s.transform = scale(2., 2., 2.);
        bencher.iter(|| s.intersect(&r));
    }
}
