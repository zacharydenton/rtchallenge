use crate::geometry::*;

pub fn intersect(ray: Ray) -> Intersections {
    let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
    let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
    let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

    let tmin = xtmin.max(ytmin).max(ztmin);
    let tmax = xtmax.min(ytmax).min(ztmax);
    let mut result = Intersections::new();

    if tmin <= tmax {
        result.push(tmin);
        result.push(tmax);
    }

    result
}

pub fn normal_at(point: Tuple4) -> Tuple4 {
    let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());
    if maxc == point.x.abs() {
        vector3(point.x, 0., 0.)
    } else if maxc == point.y.abs() {
        vector3(0., point.y, 0.)
    } else {
        vector3(0., 0., point.z)
    }
}

// Cube intersection helper.
#[inline]
fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let inv_d = direction.recip();
    let t0: f32;
    let t1: f32;
    if inv_d >= 0. {
        t0 = (-1. - origin) * inv_d;
        t1 = (1. - origin) * inv_d;
    } else {
        t1 = (-1. - origin) * inv_d;
        t0 = (1. - origin) * inv_d;
    }
    (t0, t1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn a_ray_intersects_a_cube() {
        let examples = vec![
            (point3(5., 0.5, 0.), vector3(-1., 0., 0.), 4., 6.),
            (point3(-5., 0.5, 0.), vector3(1., 0., 0.), 4., 6.),
            (point3(0.5, 5., 0.), vector3(0., -1., 0.), 4., 6.),
            (point3(0.5, -5., 0.), vector3(0., 1., 0.), 4., 6.),
            (point3(0.5, 0., 5.), vector3(0., 0., -1.), 4., 6.),
            (point3(0.5, 0., -5.), vector3(0., 0., 1.), 4., 6.),
            (point3(0., 0.5, 0.), vector3(0., 0., 1.), -1., 1.),
        ];
        for (origin, direction, t0, t1) in examples {
            let r = ray(origin, direction);
            let xs = intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs.t0, t0);
            assert_eq!(xs.t1, t1);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
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
            let xs = intersect(r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
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
            assert_eq!(normal_at(point), normal);
        }
    }

    #[bench]
    fn bench_cube_intersection(bencher: &mut Bencher) {
        let r = ray(point3(5., 0.5, 0.), vector3(-1., 0., 0.));
        bencher.iter(|| intersect(r));
    }
}
