use crate::geometry::*;

pub fn intersect(ray: Ray) -> Intersections {
    let mut result = Intersections::new();

    if ray.direction.y.abs() > 0. {
        result.push(-ray.origin.y / ray.direction.y);
    }

    result
}

pub fn normal_at(_point: Tuple4) -> Tuple4 {
    vector3(0., 1., 0.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let r = ray(point3(0., 10., 0.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let r = ray(point3(0., 1., 0.), vector3(0., -1., 0.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.t0, 1.);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let r = ray(point3(0., -1., 0.), vector3(0., 1., 0.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.t0, 1.);
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n1 = normal_at(point3(0., 0., 0.));
        let n2 = normal_at(point3(10., 0., -10.));
        let n3 = normal_at(point3(-5., 0., 150.));
        assert_eq!(n1, vector3(0., 1., 0.,));
        assert_eq!(n2, vector3(0., 1., 0.,));
        assert_eq!(n3, vector3(0., 1., 0.,));
    }
}
