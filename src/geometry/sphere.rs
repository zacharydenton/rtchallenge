use crate::geometry::*;

pub fn intersect(ray: Ray) -> Intersections {
    // The vector from the sphere's center, to the ray origin
    // (remember: the sphere is centered at the world origin)
    let sphere_to_ray = ray.origin - point3(0., 0., 0.);

    let a = ray.direction.dot(ray.direction);
    let b = 2. * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

    let mut discriminant = b * b - 4. * a * c;

    if discriminant.abs() < 1e-3 {
        discriminant = 0.;
    }

    let mut result = Intersections::new();

    if discriminant >= 0. {
        result.push((-b - discriminant.sqrt()) / (2. * a));
        result.push((-b + discriminant.sqrt()) / (2. * a));
    }

    result
}

pub fn normal_at(point: Tuple4) -> Tuple4 {
    point - point3(0., 0., 0.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.t0, 4.0);
        assert_eq!(xs.t1, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point3(0., 1., -5.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.t0, 5.0);
        assert_eq!(xs.t1, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point3(0., 2., -5.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.t0, -1.0);
        assert_eq!(xs.t1, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));
        let xs = intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.t0, -6.0);
        assert_eq!(xs.t1, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let n = normal_at(point3(1., 0., 0.));
        assert_eq!(n, vector3(1., 0., 0.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let n = normal_at(point3(0., 1., 0.));
        assert_eq!(n, vector3(0., 1., 0.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let n = normal_at(point3(0., 0., 1.));
        assert_eq!(n, vector3(0., 0., 1.,));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let root3over3 = (3 as f32).sqrt() / 3.;
        let n = normal_at(point3(root3over3, root3over3, root3over3));
        assert_approx_eq!(n.x, root3over3);
        assert_approx_eq!(n.y, root3over3);
        assert_approx_eq!(n.z, root3over3);
    }
}
