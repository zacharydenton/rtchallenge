use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Plane {},
    Sphere {},
    Cube {},
    Cone {
        /// Minimum y-value for the cone.
        min: f32,
        /// Maximum y-value for the cone.
        max: f32,
        /// Whether to close the cone on the end.
        closed: bool,
    },
    Cylinder {
        /// Minimum y-value for the cylinder.
        min: f32,
        /// Maximum y-value for the cylinder.
        max: f32,
        /// Whether to close the cylinder on each end.
        closed: bool,
    },
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

/// Constructs a unit cube centered at the origin (0, 0, 0).
pub fn cube() -> Object {
    Object {
        shape: Shape::Cube {},
        transform: I4,
        material: material(),
    }
}

/// Constructs an infinite double-napped cone centered at the origin (0, 0, 0).
pub fn cone() -> Object {
    Object {
        shape: Shape::Cone {
            min: -std::f32::INFINITY,
            max: std::f32::INFINITY,
            closed: false,
        },
        transform: I4,
        material: material(),
    }
}

/// Constructs an infinite cylinder of radius 1 centered at the origin (0, 0, 0).
pub fn cylinder() -> Object {
    Object {
        shape: Shape::Cylinder {
            min: -std::f32::INFINITY,
            max: std::f32::INFINITY,
            closed: false,
        },
        transform: I4,
        material: material(),
    }
}

/// Constructs a unit sphere centered at the origin (0, 0, 0) with a glassy material.
pub fn glass_sphere() -> Object {
    let mut glass_material = material();
    glass_material.transparency = 1.0;
    glass_material.refractive_index = 1.5;
    Object {
        shape: Shape::Sphere {},
        transform: I4,
        material: glass_material,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Object,
    pub point: Option<Tuple4>,
    pub over_point: Option<Tuple4>,
    pub under_point: Option<Tuple4>,
    pub eyev: Option<Tuple4>,
    pub normalv: Option<Tuple4>,
    pub reflectv: Option<Tuple4>,
    pub inside: Option<bool>,
    pub n1: Option<f32>,
    pub n2: Option<f32>,
}

fn intersection(t: f32, object: &Object) -> Intersection {
    Intersection {
        t,
        object,
        point: None,
        over_point: None,
        under_point: None,
        eyev: None,
        normalv: None,
        reflectv: None,
        inside: None,
        n1: None,
        n2: None,
    }
}

impl Intersection<'_> {
    #[inline]
    fn prepare(
        &mut self,
        ray: &Ray,
        inverse_transform: Matrix4,
        all_intersections: &Intersections,
    ) -> Self {
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
        self.under_point = Some(point - normalv * 1e-2);

        let mut containers: Vec<&Object> = vec![];
        for i in all_intersections {
            if i.t == self.t && i.object == self.object {
                if containers.len() == 0 {
                    self.n1 = Some(1.0);
                } else {
                    self.n1 = Some(containers.last().unwrap().material.refractive_index);
                }
            }

            if containers.contains(&i.object) {
                containers.retain(|o| o != &i.object);
            } else {
                containers.push(i.object);
            }

            if i.t == self.t && i.object == self.object {
                if containers.len() == 0 {
                    self.n2 = Some(1.0);
                } else {
                    self.n2 = Some(containers.last().unwrap().material.refractive_index);
                }
                break;
            }
        }

        *self
    }
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

/// Computes the Schlick approximation for the given intersection.
pub fn schlick(intersection: &Intersection) -> f32 {
    let n1 = intersection.n1.unwrap();
    let n2 = intersection.n2.unwrap();
    let mut cos = intersection
        .eyev
        .unwrap()
        .dot(intersection.normalv.unwrap());

    if n1 > n2 {
        let n = n1 / n2;
        let sin2_t = n * n * (1. - cos * cos);

        if sin2_t > 1.0 {
            // Total internal reflection.
            return 1.0;
        }

        let cos_t = (1. - sin2_t).sqrt();
        cos = cos_t;
    }

    let r = (n1 - n2) / (n1 + n2);
    let r0 = r * r;

    r0 + (1. - r0) * (1. - cos).powi(5)
}

// Cube intersection helper.
fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1. - origin;
    let tmax_numerator = 1. - origin;

    let (tmin, tmax) = if direction.abs() >= 1e-2 {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * std::f32::INFINITY,
            tmax_numerator * std::f32::INFINITY,
        )
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

// Helper to reduce duplication in capped cone and cylinder intersection.
fn check_cap(ray: &Ray, t: f32, radius: f32) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    x * x + z * z <= radius + 1e-2
}

// Helper which adds capped cylinder and cone intersections.
fn intersect_caps<'a>(object: &'a Object, ray: &Ray, xs: &mut Intersections<'a>) {
    if let Shape::Cylinder { min, max, closed } = object.shape {
        if !closed || ray.direction.y.abs() < 1e-2 {
            // Caps only matter if the cylinder is closed, and might possibly be intersected by the
            // ray.
            return;
        }

        // Check for an intersection with the lower end cap by intersecting the ray
        // with the plane at y = min.
        let tmin = (min - ray.origin.y) / ray.direction.y;
        if check_cap(ray, tmin, 1.) {
            xs.push(intersection(tmin, object));
        }

        // Check for an intersection with the upper end cap by intersectin the ray
        // with the plane y = max.
        let tmax = (max - ray.origin.y) / ray.direction.y;
        if check_cap(ray, tmax, 1.) {
            xs.push(intersection(tmax, object));
        }
    } else if let Shape::Cone { min, max, closed } = object.shape {
        if !closed || ray.direction.y.abs() < 1e-2 {
            // Caps only matter if the cone is closed, and might possibly be intersected by the
            // ray.
            return;
        }

        // Check for an intersection with the lower end cap by intersecting the ray
        // with the plane at y = min.
        let tmin = (min - ray.origin.y) / ray.direction.y;
        if check_cap(ray, tmin, min.abs()) {
            xs.push(intersection(tmin, object));
        }

        // Check for an intersection with the upper end cap by intersectin the ray
        // with the plane y = max.
        let tmax = (max - ray.origin.y) / ray.direction.y;
        if check_cap(ray, tmax, max.abs()) {
            xs.push(intersection(tmax, object));
        }
    } else {
        panic!("Expected a cone or cylinder.");
    }
}

impl Object {
    /// Returns the collection of Intersections where the ray intersects the object.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        // Instead of transforming the object, apply the inverse transformation to the ray.
        let inverse_transform = self.transform.inverse();
        let transformed_ray = ray.transform(inverse_transform);

        let mut result = match self.shape {
            Shape::Plane {} => {
                if transformed_ray.direction.y.abs() < 1e-2 {
                    return vec![];
                }

                let t = -transformed_ray.origin.y / transformed_ray.direction.y;
                vec![intersection(t, self)]
            }
            Shape::Sphere {} => {
                // The vector from the sphere's center, to the ray origin
                // (remember: the sphere is centered at the world origin)
                let sphere_to_ray = transformed_ray.origin - point3(0., 0., 0.);

                let a = transformed_ray.direction.dot(transformed_ray.direction);
                let b = 2. * transformed_ray.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

                let mut discriminant = b * b - 4. * a * c;

                if discriminant.abs() < 1e-5 {
                    discriminant = 0.;
                }

                if discriminant < 0. {
                    vec![]
                } else {
                    let t1 = (-b - discriminant.sqrt()) / (2. * a);
                    let t2 = (-b + discriminant.sqrt()) / (2. * a);
                    vec![intersection(t1, self), intersection(t2, self)]
                }
            }
            Shape::Cube {} => {
                let (xtmin, xtmax) =
                    check_axis(transformed_ray.origin.x, transformed_ray.direction.x);
                let (ytmin, ytmax) =
                    check_axis(transformed_ray.origin.y, transformed_ray.direction.y);
                let (ztmin, ztmax) =
                    check_axis(transformed_ray.origin.z, transformed_ray.direction.z);

                let tmin = xtmin.max(ytmin).max(ztmin);
                let tmax = xtmax.min(ytmax).min(ztmax);

                if tmin > tmax {
                    vec![]
                } else {
                    vec![intersection(tmin, self), intersection(tmax, self)]
                }
            }
            Shape::Cone {
                min,
                max,
                closed: _,
            } => {
                let mut result = vec![];

                let a = (transformed_ray.direction.x * transformed_ray.direction.x)
                    - (transformed_ray.direction.y * transformed_ray.direction.y)
                    + (transformed_ray.direction.z * transformed_ray.direction.z);
                let b = 2. * transformed_ray.origin.x * transformed_ray.direction.x
                    - 2. * transformed_ray.origin.y * transformed_ray.direction.y
                    + 2. * transformed_ray.origin.z * transformed_ray.direction.z;
                let c = (transformed_ray.origin.x * transformed_ray.origin.x)
                    - (transformed_ray.origin.y * transformed_ray.origin.y)
                    + (transformed_ray.origin.z * transformed_ray.origin.z);

                if a.abs() < 1e-3 {
                    if b.abs() < 1e-3 {
                        return vec![];
                    }

                    let t = -c / (2. * b);
                    result.push(intersection(t, self));
                } else {
                    let mut discriminant = b * b - 4. * a * c;

                    if discriminant.abs() < 1e-5 {
                        discriminant = 0.;
                    }

                    if discriminant < 0. {
                        // Ray does not intersect the cone.
                        return vec![];
                    }

                    let (tmin, tmax) = {
                        let t0 = (-b - discriminant.sqrt()) / (2. * a);
                        let t1 = (-b + discriminant.sqrt()) / (2. * a);
                        if t0 < t1 {
                            (t0, t1)
                        } else {
                            (t1, t0)
                        }
                    };

                    let ymin = transformed_ray.origin.y + tmin * ray.direction.y;
                    if min < ymin && ymin < max {
                        result.push(intersection(tmin, self));
                    }

                    let ymax = transformed_ray.origin.y + tmax * ray.direction.y;
                    if min < ymax && ymax < max {
                        result.push(intersection(tmax, self));
                    }
                }

                intersect_caps(self, &transformed_ray, &mut result);

                result
            }
            Shape::Cylinder {
                min,
                max,
                closed: _,
            } => {
                let mut result = vec![];

                let a = (transformed_ray.direction.x * transformed_ray.direction.x)
                    + (transformed_ray.direction.z * transformed_ray.direction.z);

                if a.abs() > 1e-3 {
                    // Ray is not parallel to the y-axis.

                    let b = 2. * transformed_ray.origin.x * transformed_ray.direction.x
                        + 2. * transformed_ray.origin.z * transformed_ray.direction.z;
                    let c = (transformed_ray.origin.x * transformed_ray.origin.x)
                        + (transformed_ray.origin.z * transformed_ray.origin.z)
                        - 1.;
                    let mut discriminant = b * b - 4. * a * c;

                    if discriminant.abs() < 1e-5 {
                        discriminant = 0.;
                    }

                    if discriminant < 0. {
                        // Ray does not intersect the cylinder.
                        return vec![];
                    }

                    let (tmin, tmax) = {
                        let t0 = (-b - discriminant.sqrt()) / (2. * a);
                        let t1 = (-b + discriminant.sqrt()) / (2. * a);
                        if t0 < t1 {
                            (t0, t1)
                        } else {
                            (t1, t0)
                        }
                    };

                    let ymin = transformed_ray.origin.y + tmin * ray.direction.y;
                    if min < ymin && ymin < max {
                        result.push(intersection(tmin, self));
                    }

                    let ymax = transformed_ray.origin.y + tmax * ray.direction.y;
                    if min < ymax && ymax < max {
                        result.push(intersection(tmax, self));
                    }
                }

                intersect_caps(self, &transformed_ray, &mut result);

                result
            }
            Shape::TestShape {} => vec![],
        };

        let all_intersections = result.clone();
        result
            .iter_mut()
            .map(|intersection| intersection.prepare(ray, inverse_transform, &all_intersections))
            .collect()
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
            Shape::Cube {} => {
                let object_point = inverse_transform * point;
                let maxc = object_point
                    .x
                    .abs()
                    .max(object_point.y.abs())
                    .max(object_point.z.abs());
                if maxc == object_point.x.abs() {
                    vector3(object_point.x, 0., 0.)
                } else if maxc == object_point.y.abs() {
                    vector3(0., object_point.y, 0.)
                } else {
                    vector3(0., 0., object_point.z)
                }
            }
            Shape::Cone {
                min,
                max,
                closed: _,
            } => {
                let object_point = inverse_transform * point;

                // The square of the distance from the y axis.
                let d2 = object_point.x * object_point.x + object_point.z * object_point.z;

                if d2 < max.abs() && object_point.y >= max - 1e-2 {
                    // Hitting the top cap.
                    vector3(0., 1., 0.)
                } else if d2 < min.abs() && object_point.y <= min + 1e-2 {
                    // Hitting the bottom cap.
                    vector3(0., -1., 0.)
                } else {
                    let mut y =
                        (object_point.x * object_point.x + object_point.z * object_point.z).sqrt();

                    if object_point.y > 0. {
                        y = -y;
                    }

                    vector3(object_point.x, y, object_point.z)
                }
            }
            Shape::Cylinder {
                min,
                max,
                closed: _,
            } => {
                let object_point = inverse_transform * point;

                // The square of the distance from the y axis.
                let d2 = object_point.x * object_point.x + object_point.z * object_point.z;

                if d2 < 1. && object_point.y >= max - 1e-2 {
                    // Hitting the top cap.
                    vector3(0., 1., 0.)
                } else if d2 < 1. && object_point.y <= min + 1e-2 {
                    // Hitting the bottom cap.
                    vector3(0., -1., 0.)
                } else {
                    vector3(object_point.x, 0., object_point.z)
                }
            }
            Shape::Sphere {} | Shape::TestShape {} => {
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
    fn a_ray_intersects_a_cube() {
        let c = cube();
        let examples = vec![
            (point3(5., 0.5, 0.), vector3(-1., 0., 0.), 4., 6.),
            (point3(-5., 0.5, 0.), vector3(1., 0., 0.), 4., 6.),
            (point3(0.5, 5., 0.), vector3(0., -1., 0.), 4., 6.),
            (point3(0.5, -5., 0.), vector3(0., 1., 0.), 4., 6.),
            (point3(0.5, 0., 5.), vector3(0., 0., -1.), 4., 6.),
            (point3(0.5, 0., -5.), vector3(0., 0., 1.), 4., 6.),
            (point3(0., 0.5, 0.), vector3(0., 0., 1.), -1., 1.),
        ];
        for (origin, direction, t1, t2) in examples {
            let r = ray(origin, direction);
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t1);
            assert_eq!(xs[1].t, t2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = cube();
        let examples = vec![
            (point3(-2., 0., 0.), vector3(0.2673, 0.5345, 0.8018)),
            (point3(0., -2., 0.), vector3(0.8018, 0.2673, 0.5345)),
            (point3(0., 0., -2.), vector3(0.5345, 0.8018, 0.2673)),
            (point3(2., 0., 2.), vector3(0., 0., -1.)),
            (point3(0., 2., 2.), vector3(0., -1., 0.)),
            (point3(2., 2., 0.), vector3(-1., 0., 0.)),
        ];
        for (origin, direction) in examples {
            let r = ray(origin, direction);
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = cube();
        let examples = vec![
            (point3(1., 0.5, -0.8), vector3(1., 0., 0.)),
            (point3(-1., -0.2, 0.9), vector3(-1., 0., 0.)),
            (point3(-0.4, 1., -0.1), vector3(0., 1., 0.)),
            (point3(0.3, -1., -0.7), vector3(0., -1., 0.)),
            (point3(-0.6, 0.3, 1.), vector3(0., 0., 1.)),
            (point3(0.4, 0.4, -1.), vector3(0., 0., -1.)),
            (point3(1., 1., 1.), vector3(1., 0., 0.)),
            (point3(-1., -1., -1.), vector3(-1., 0., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(c.normal(point), normal);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = cylinder();
        let examples = vec![
            (point3(1., 0., -5.), vector3(0., 0., 1.), 5., 5.),
            (point3(0., 0., -5.), vector3(0., 0., 1.), 4., 6.),
            (point3(0.5, 0., -5.), vector3(0.1, 1., 1.), 6.80798, 7.08872),
        ];
        for (origin, direction, t0, t1) in examples {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_approx_eq!(xs[0].t, t0, 1e-4);
            assert_approx_eq!(xs[1].t, t1, 1e-4);
        }
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = cylinder();
        let examples = vec![
            (point3(1., 0., 0.), vector3(0., 1., 0.)),
            (point3(0., 0., 0.), vector3(0., 1., 0.)),
            (point3(0., 0., -5.), vector3(1., 1., 1.)),
        ];
        for (origin, direction) in examples {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinder() {
        let cyl = cylinder();
        let examples = vec![
            (point3(1., 0., 0.), vector3(1., 0., 0.)),
            (point3(0., 5., -1.), vector3(0., 0., -1.)),
            (point3(0., -2., 1.), vector3(0., 0., 1.)),
            (point3(-1., 1., 0.), vector3(-1., 0., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(cyl.normal(point), normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = cylinder();
        if let Shape::Cylinder {
            min,
            max,
            closed: _,
        } = cyl.shape
        {
            assert_eq!(min, -std::f32::INFINITY);
            assert_eq!(max, std::f32::INFINITY);
        } else {
            panic!();
        }
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let mut cyl = cylinder();
        cyl.shape = Shape::Cylinder {
            min: 1.,
            max: 2.,
            closed: false,
        };
        let examples = vec![
            (point3(0., 1.5, 0.), vector3(0.1, 1., 0.), 0),
            (point3(0., 3., -5.), vector3(0., 0., 1.), 0),
            (point3(0., 0., -5.), vector3(0., 0., 1.), 0),
            (point3(0., 2., -5.), vector3(0., 0., 1.), 0),
            (point3(0., 1., -5.), vector3(0., 0., 1.), 0),
            (point3(0., 1.5, -2.), vector3(0., 0., 1.), 2),
        ];
        for (point, direction, count) in examples {
            let r = ray(point, direction.normalize());
            let xs = cyl.intersect(&r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = cylinder();
        if let Shape::Cylinder {
            min: _,
            max: _,
            closed,
        } = cyl.shape
        {
            assert_eq!(closed, false);
        } else {
            panic!();
        }
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = cylinder();
        cyl.shape = Shape::Cylinder {
            min: 1.,
            max: 2.,
            closed: true,
        };
        let examples = vec![
            (point3(0., 3., 0.), vector3(0., -1., 0.), 2),
            (point3(0., 3., -2.), vector3(0., -1., 2.), 2),
            (point3(0., 4., -2.), vector3(0., -1., 1.), 2),
            (point3(0., 0., -2.), vector3(0., 1., 2.), 2),
            (point3(0., -1., -2.), vector3(0., 1., 1.), 2),
        ];
        for (point, direction, count) in examples {
            let r = ray(point, direction.normalize());
            let xs = cyl.intersect(&r);
            assert_eq!(xs.len(), count, "{:?}", xs);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let mut cyl = cylinder();
        cyl.shape = Shape::Cylinder {
            min: 1.,
            max: 2.,
            closed: true,
        };
        let examples = vec![
            (point3(0., 1., 0.), vector3(0., -1., 0.)),
            (point3(0.5, 1., 0.), vector3(0., -1., 0.)),
            (point3(0., 1., 0.5), vector3(0., -1., 0.)),
            (point3(0., 2., 0.), vector3(0., 1., 0.)),
            (point3(0.5, 2., 0.), vector3(0., 1., 0.)),
            (point3(0., 2., 0.5), vector3(0., 1., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(cyl.normal(point), normal, "{:?}", point);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let shape = cone();
        let examples = vec![
            (point3(0., 0., -5.), vector3(0., 0., 1.), 5., 5.),
            (point3(0., 0., -5.), vector3(1., 1., 1.), 8.66025, 8.66025),
            (
                point3(1., 1., -5.),
                vector3(-0.5, -1., 1.),
                4.55006,
                49.44994,
            ),
        ];
        for (origin, direction, t0, t1) in examples {
            let r = ray(origin, direction.normalize());
            let xs = shape.intersect(&r);
            assert_eq!(xs.len(), 2, "{:?}", direction);
            assert_approx_eq!(xs[0].t, t0, 1e-3);
            assert_approx_eq!(xs[1].t, t1, 1e-3);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = cone();
        let direction = vector3(0., 1., 1.).normalize();
        let r = ray(point3(0., 0., -1.), direction);
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_approx_eq!(xs[0].t, 0.35355, 1e-3);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let mut con = cone();
        con.shape = Shape::Cone {
            min: -0.5,
            max: 0.5,
            closed: true,
        };
        let examples = vec![
            (point3(0., 0., -5.), vector3(0., 1., 0.), 0),
            (point3(0., 0., -0.25), vector3(0., 1., 1.), 2),
            (point3(0., 0., -0.25), vector3(0., 1., 0.), 4),
        ];
        for (origin, direction, count) in examples {
            let r = ray(origin, direction.normalize());
            let xs = con.intersect(&r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = cone();
        let examples = vec![
            (point3(0., 0., 0.), vector3(0., 0., 0.)),
            (
                point3(1., 1., 1.),
                vector3(1., -std::f32::consts::SQRT_2, 1.),
            ),
            (point3(-1., -1., 0.), vector3(-1., 1., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(shape.normal(point), normal.normalize());
        }
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
        i.prepare(&r, shape.transform.inverse(), &vec![i]);
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
    fn the_hit_should_compute_the_under_point() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut shape = glass_sphere();
        shape.transform = translate(0., 0., 1.);
        let mut i = intersection(5., &shape);
        let xs = vec![i];
        i.prepare(&r, i.object.transform.inverse(), &xs);
        assert!(i.under_point.unwrap().z > 5e-3);
        assert!(i.point.unwrap().z < i.under_point.unwrap().z);
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

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();
        assert_eq!(s.transform, I4);
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.transform = scale(2., 2., 2.);
        a.material.refractive_index = 1.5;
        let mut b = glass_sphere();
        b.transform = translate(0., 0., -0.25);
        b.material.refractive_index = 2.0;
        let mut c = glass_sphere();
        c.transform = translate(0., 0., 0.25);
        c.material.refractive_index = 2.5;
        let r = ray(point3(0., 0., -4.), vector3(0., 0., 1.));
        let mut xs = vec![
            intersection(2., &a),
            intersection(2.75, &b),
            intersection(3.25, &c),
            intersection(4.75, &b),
            intersection(5.25, &c),
            intersection(6., &a),
        ];
        let all_intersections = xs.clone();
        let expected = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (intersection, (n1, n2)) in xs.iter_mut().zip(expected) {
            intersection.prepare(
                &r,
                intersection.object.transform.inverse(),
                &all_intersections,
            );
            assert_eq!(intersection.n1.unwrap(), n1);
            assert_eq!(intersection.n2.unwrap(), n2);
        }
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(
            point3(0., 0., -std::f32::consts::SQRT_2 * 0.5),
            vector3(0., 1., 0.),
        );
        let mut xs = vec![
            intersection(-std::f32::consts::SQRT_2 * 0.5, &shape),
            intersection(std::f32::consts::SQRT_2 * 0.5, &shape),
        ];
        let all_intersections = xs.clone();
        let i = xs.last_mut().unwrap();
        i.prepare(&r, i.object.transform.inverse(), &all_intersections);
        let reflectance = schlick(&i);
        assert_approx_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(point3(0., 0., 0.), vector3(0., 1., 0.));
        let mut xs = vec![intersection(-1., &shape), intersection(1., &shape)];
        let all_intersections = xs.clone();
        let i = xs.last_mut().unwrap();
        i.prepare(&r, i.object.transform.inverse(), &all_intersections);
        let reflectance = schlick(&i);
        assert_approx_eq!(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = glass_sphere();
        let r = ray(point3(0., 0.99, -2.), vector3(0., 0., 1.));
        let mut xs = vec![intersection(1.8589, &shape)];
        let all_intersections = xs.clone();
        let i = xs.first_mut().unwrap();
        i.prepare(&r, i.object.transform.inverse(), &all_intersections);
        let reflectance = schlick(&i);
        assert_approx_eq!(reflectance, 0.48873);
    }

    #[bench]
    fn bench_intersecting_a_scaled_sphere_with_a_ray(bencher: &mut Bencher) {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut s = sphere();
        s.transform = scale(2., 2., 2.);
        bencher.iter(|| s.intersect(&r));
    }
}
