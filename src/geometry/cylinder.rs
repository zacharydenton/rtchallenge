use crate::geometry::*;

pub fn intersect(ray: Ray, min: f32, max: f32, closed: bool) -> Intersections {
    let mut result = Intersections::new();

    let a = (ray.direction.x * ray.direction.x) + (ray.direction.z * ray.direction.z);

    if a.abs() > 1e-3 {
        // Ray is not parallel to the y-axis.

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = (ray.origin.x * ray.origin.x) + (ray.origin.z * ray.origin.z) - 1.;
        let mut discriminant = b * b - 4. * a * c;

        if discriminant.abs() < 1e-3 {
            discriminant = 0.;
        }

        if discriminant < 0. {
            // Ray does not intersect the cylinder.
            return result;
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

        let ymin = ray.origin.y + tmin * ray.direction.y;
        if min < ymin && ymin < max {
            result.push(tmin);
        }

        let ymax = ray.origin.y + tmax * ray.direction.y;
        if min < ymax && ymax < max {
            result.push(tmax);
        }
    }

    intersect_caps(ray, &mut result, min, max, closed);

    result
}

pub fn normal_at(point: Tuple4, min: f32, max: f32, _closed: bool) -> Tuple4 {
    // The square of the distance from the y axis.
    let d2 = point.x * point.x + point.z * point.z;

    if d2 < 1. && point.y >= max - 1e-3 {
        // Hitting the top cap.
        vector3(0., 1., 0.)
    } else if d2 < 1. && point.y <= min + 1e-3 {
        // Hitting the bottom cap.
        vector3(0., -1., 0.)
    } else {
        vector3(point.x, 0., point.z)
    }
}

// Helper to reduce duplication in capped cylinder intersection.
fn check_cap(ray: Ray, t: f32) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    x * x + z * z <= 1. + 1e-3
}

// Helper which adds capped cylinder intersections.
fn intersect_caps(ray: Ray, xs: &mut Intersections, min: f32, max: f32, closed: bool) {
    if !closed || ray.direction.y.abs() < 1e-3 {
        // Caps only matter if the cylinder is closed, and might possibly be intersected
        // by the ray.
        return;
    }

    // Check for an intersection with the lower end cap by intersecting the ray
    // with the plane at y = min.
    let tmin = (min - ray.origin.y) / ray.direction.y;
    if check_cap(ray, tmin) {
        xs.push(tmin);
    }

    // Check for an intersection with the upper end cap by intersectin the ray
    // with the plane y = max.
    let tmax = (max - ray.origin.y) / ray.direction.y;
    if check_cap(ray, tmax) {
        xs.push(tmax);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let examples = vec![
            (point3(1., 0., -5.), vector3(0., 0., 1.), 5., 5.),
            (point3(0., 0., -5.), vector3(0., 0., 1.), 4., 6.),
            (point3(0.5, 0., -5.), vector3(0.1, 1., 1.), 6.80798, 7.08872),
        ];
        for (origin, direction, t0, t1) in examples {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = intersect(r, -std::f32::INFINITY, std::f32::INFINITY, false);
            assert_eq!(xs.len(), 2);
            assert_approx_eq!(xs.t0, t0, 1e-4);
            assert_approx_eq!(xs.t1, t1, 1e-4);
        }
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        let examples = vec![
            (point3(1., 0., 0.), vector3(0., 1., 0.)),
            (point3(0., 0., 0.), vector3(0., 1., 0.)),
            (point3(0., 0., -5.), vector3(1., 1., 1.)),
        ];
        for (origin, direction) in examples {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = intersect(r, -std::f32::INFINITY, std::f32::INFINITY, false);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinder() {
        let examples = vec![
            (point3(1., 0., 0.), vector3(1., 0., 0.)),
            (point3(0., 5., -1.), vector3(0., 0., -1.)),
            (point3(0., -2., 1.), vector3(0., 0., 1.)),
            (point3(-1., 1., 0.), vector3(-1., 0., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(
                normal_at(point, -std::f32::INFINITY, std::f32::INFINITY, false),
                normal
            );
        }
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
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
            let xs = intersect(r, 1., 2., false);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let examples = vec![
            (point3(0., 3., 0.), vector3(0., -1., 0.), 2),
            (point3(0., 3., -2.), vector3(0., -1., 2.), 2),
            (point3(0., 4., -2.), vector3(0., -1., 1.), 2),
            (point3(0., 0., -2.), vector3(0., 1., 2.), 2),
            (point3(0., -1., -2.), vector3(0., 1., 1.), 2),
        ];
        for (point, direction, count) in examples {
            let r = ray(point, direction.normalize());
            let xs = intersect(r, 1., 2., true);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let examples = vec![
            (point3(0., 1., 0.), vector3(0., -1., 0.)),
            (point3(0.5, 1., 0.), vector3(0., -1., 0.)),
            (point3(0., 1., 0.5), vector3(0., -1., 0.)),
            (point3(0., 2., 0.), vector3(0., 1., 0.)),
            (point3(0.5, 2., 0.), vector3(0., 1., 0.)),
            (point3(0., 2., 0.5), vector3(0., 1., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(normal_at(point, 1., 2., true), normal, "{:?}", point);
        }
    }
}
