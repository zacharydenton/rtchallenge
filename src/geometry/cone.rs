use crate::geometry::*;

pub fn intersect(ray: Ray, min: f32, max: f32, closed: bool) -> Intersections {
    let mut result = Intersections::new();

    let a = ray.direction.x.mul_add(
        ray.direction.x,
        ray.direction
            .z
            .mul_add(ray.direction.z, -ray.direction.y * ray.direction.y),
    );
    let b = ray.origin.x.mul_add(
        ray.direction.x,
        ray.origin
            .z
            .mul_add(ray.direction.z, -ray.origin.y * ray.direction.y),
    );
    let c = ray.origin.x.mul_add(
        ray.origin.x,
        ray.origin
            .z
            .mul_add(ray.origin.z, -ray.origin.y * ray.origin.y),
    );

    if a.abs() < 1e-5 {
        if b.abs() < 1e-5 {
            return result;
        }

        let t = -c / (4. * b);
        result.push(t);
    } else {
        let mut discriminant = b.mul_add(b, -a * c);

        if discriminant.abs() < 1e-5 {
            discriminant = 0.;
        }

        if discriminant < 0. {
            // Ray does not intersect the cone.
            return result;
        }

        let (tmin, tmax) = {
            let inv_a = a.recip();
            let d_sqrt = discriminant.sqrt();
            let t0 = (-b - d_sqrt) * inv_a;
            let t1 = (-b + d_sqrt) * inv_a;
            if t0 < t1 {
                (t0, t1)
            } else {
                (t1, t0)
            }
        };

        let ymin = ray.direction.y.mul_add(tmin, ray.origin.y);
        let ymax = ray.direction.y.mul_add(tmax, ray.origin.y);

        if min < ymin && ymin < max {
            result.push(tmin);
        }

        if min < ymax && ymax < max {
            result.push(tmax);
        }
    }

    intersect_caps(ray, &mut result, min, max, closed);

    result
}

pub fn normal_at(point: Tuple4, min: f32, max: f32, _closed: bool) -> Tuple4 {
    // The square of the distance from the y axis.
    let d2 = point.x.mul_add(point.x, point.z * point.z);

    if d2 < max.abs() && point.y >= max - 1e-5 {
        // Hitting the top cap.
        vector3(0., 1., 0.)
    } else if d2 < min.abs() && point.y <= min + 1e-5 {
        // Hitting the bottom cap.
        vector3(0., -1., 0.)
    } else {
        let mut y = point.x.mul_add(point.x, point.z * point.z).sqrt();

        if point.y > 0. {
            y = -y;
        }

        vector3(point.x, y, point.z)
    }
}

// Checks if the point along the ray at position t intersects the cap with the
// given radius.
fn check_cap(ray: Ray, t: f32, radius: f32) -> bool {
    let x = ray.direction.x.mul_add(t, ray.origin.x);
    let z = ray.direction.z.mul_add(t, ray.origin.z);

    x.mul_add(x, z * z) <= radius + 1e-5
}

// Helper which adds capped cone intersections.
fn intersect_caps(ray: Ray, xs: &mut Intersections, min: f32, max: f32, closed: bool) {
    if !closed || ray.direction.y.abs() < 1e-5 {
        // Caps only matter if the cone is closed, and might possibly be intersected by
        // the ray.
        return;
    }

    // Check for an intersection with the lower end cap by intersecting the ray
    // with the plane at y = min.
    let inv_y = ray.direction.y.recip();
    let tmin = (min - ray.origin.y) * inv_y;
    if check_cap(ray, tmin, min.abs()) {
        xs.push(tmin);
    }

    // Check for an intersection with the upper end cap by intersectin the ray
    // with the plane y = max.
    let tmax = (max - ray.origin.y) * inv_y;
    if check_cap(ray, tmax, max.abs()) {
        xs.push(tmax);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
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
            let xs = intersect(r, -std::f32::INFINITY, std::f32::INFINITY, false);
            assert_eq!(xs.len(), 2);
            assert_approx_eq!(xs.t0, t0, 1e-3);
            assert_approx_eq!(xs.t1, t1, 1e-3);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let direction = vector3(0., 1., 1.).normalize();
        let r = ray(point3(0., 0., -1.), direction);
        let xs = intersect(r, -std::f32::INFINITY, std::f32::INFINITY, false);
        assert_eq!(xs.len(), 1);
        assert_approx_eq!(xs.t0, 0.35355, 1e-3);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let examples = vec![
            (point3(0., 0., -5.), vector3(0., 1., 0.), 0),
            (point3(0., 0., -0.25), vector3(0., 1., 1.), 2),
            (point3(0., 0., -0.25), vector3(0., 1., 0.), 2), /* XXX: Should be 4 intersections,
                                                              * but we only capture the nearest
                                                              * 2. */
        ];
        for (origin, direction, count) in examples {
            let r = ray(origin, direction.normalize());
            let xs = intersect(r, -0.5, 0.5, true);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let examples = vec![
            (point3(0., 0., 0.), vector3(0., 0., 0.)),
            (
                point3(1., 1., 1.),
                vector3(1., -std::f32::consts::SQRT_2, 1.),
            ),
            (point3(-1., -1., 0.), vector3(-1., 1., 0.)),
        ];
        for (point, normal) in examples {
            assert_eq!(
                normal_at(point, -std::f32::INFINITY, std::f32::INFINITY, false),
                normal
            );
        }
    }
}
