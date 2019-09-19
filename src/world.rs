use crate::color::*;
use crate::light::*;
use crate::object::*;
use crate::ray::*;
use crate::transform::*;
use crate::tuple::*;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub max_reflections: i32,
}

pub fn world() -> World {
    World {
        objects: vec![],
        lights: vec![],
        max_reflections: 5,
    }
}

pub fn default_world() -> World {
    let mut s1 = sphere();
    s1.material.color = color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = sphere();
    s2.transform = scale(0.5, 0.5, 0.5);

    let light = point_light(point3(-10., 10., -10.), color(1., 1., 1.));

    World {
        objects: vec![s1, s2],
        lights: vec![light],
        max_reflections: 5,
    }
}

impl World {
    /// Iterates over all of the objects in the world, intersects each of them
    /// with the ray, and returns the intersections in sorted order.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .filter(|i| i.t >= 0.)
            .collect::<Intersections>();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    /// Returns the color at the given intersection.
    pub fn shade(&self, intersection: &Intersection, remaining: i32) -> Color {
        let surface = self
            .lights
            .iter()
            .map(|light| {
                intersection.object.material.lighting(
                    &intersection.object,
                    light,
                    &intersection.point.unwrap(),
                    &intersection.eyev.unwrap(),
                    &intersection.normalv.unwrap(),
                    self.is_shadowed(intersection.over_point.unwrap()),
                )
            })
            .fold(color(0., 0., 0.), |acc, x| acc + x);

        let reflection = self.reflected_color(intersection, remaining);

        surface + reflection
    }

    /// Intersects the ray with the world and returns the color at the resulting
    /// intersection.
    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_remaining(ray, self.max_reflections).clamp()
    }

    fn color_at_remaining(&self, ray: &Ray, remaining: i32) -> Color {
        let intersections = self.intersect(ray);

        if intersections.len() == 0 {
            return color(0., 0., 0.);
        }

        self.shade(&intersections[0], remaining)
    }

    /// Whether the given point is considered to be in shadow.
    pub fn is_shadowed(&self, point: Tuple4) -> bool {
        // TODO: Check more than just the first light source.
        let light = &self.lights[0];
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = ray(point, direction);
        let intersections = self.intersect(&r);

        intersections.len() > 0 && intersections[0].t < distance
    }

    /// Returns the color of the reflection at the given intersection.
    ///
    /// Will only cast reflection rays if there is remaining depth.
    pub fn reflected_color(&self, intersection: &Intersection, remaining: i32) -> Color {
        if intersection.object.material.reflective > 0. && remaining > 0 {
            let reflect_ray = ray(
                intersection.over_point.unwrap(),
                intersection.reflectv.unwrap(),
            );
            let reflect_color = self.color_at_remaining(&reflect_ray, remaining - 1);
            reflect_color * intersection.object.material.reflective
        } else {
            color(0., 0., 0.)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use test::Bencher;

    #[test]
    fn creating_a_world() {
        let w = world();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let w = default_world();
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs = w.intersect(&r);
        let i = xs.iter().find(|x| x.t == 4.).unwrap();
        let c = w.shade(&i, 1);

        assert_approx_eq!(c.r, 0.38066, 1e-5);
        assert_approx_eq!(c.g, 0.47583, 1e-5);
        assert_approx_eq!(c.b, 0.2855, 1e-5);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights = vec![point_light(point3(0., 0.25, 0.), color(1., 1., 1.))];
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let xs = w.intersect(&r);
        let i = xs.iter().find(|x| x.t == 0.5).unwrap();
        let c = w.shade(&i, 1);

        assert_approx_eq!(c.r, 0.90498, 1e-5);
        assert_approx_eq!(c.g, 0.90498, 1e-5);
        assert_approx_eq!(c.b, 0.90498, 1e-5);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 1., 0.));
        let c = w.color_at(&r);

        assert_approx_eq!(c.r, 0.0, 1e-5);
        assert_approx_eq!(c.g, 0.0, 1e-5);
        assert_approx_eq!(c.b, 0.0, 1e-5);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let c = w.color_at(&r);

        assert_approx_eq!(c.r, 0.38066, 1e-5);
        assert_approx_eq!(c.g, 0.47583, 1e-5);
        assert_approx_eq!(c.b, 0.2855, 1e-5);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = ray(point3(0., 0., 0.75), vector3(0., 0., -1.));
        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point3(0., 10., 0.);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point3(10., -10., 10.);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point3(-20., 20., -20.);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_bheind_the_point() {
        let w = default_world();
        let p = point3(-2., 2., -2.);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_is_given_an_intersection_in_shadow() {
        let mut w = world();
        w.lights
            .push(point_light(point3(0., 0., -10.), color(1., 1., 1.)));
        let s1 = sphere();
        w.objects.push(s1);
        let mut s2 = sphere();
        s2.transform = translate(0., 0., 10.);
        w.objects.push(s2);
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));

        let xs = w.intersect(&r);
        let i = xs.iter().find(|x| x.t == 4.).unwrap();
        let c = w.shade(&i, 1);

        assert_approx_eq!(c.r, 0.1, 1e-5);
        assert_approx_eq!(c.g, 0.1, 1e-5);
        assert_approx_eq!(c.b, 0.1, 1e-5);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = default_world();
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let mut shape = &mut w.objects[1];
        shape.material.ambient = 1.0;
        let xs = w.intersect(&r);
        let i = xs.iter().find(|x| x.t == 1.).unwrap();
        assert_eq!(w.reflected_color(&i, 1), color(0., 0., 0.));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translate(0., -1., 0.);
        w.objects.push(shape);
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );
        let xs = w.intersect(&r);
        let i = xs
            .iter()
            .find(|x| (x.t - std::f32::consts::SQRT_2).abs() < 1e-2)
            .unwrap();
        let reflected = w.reflected_color(&i, 1);
        assert_approx_eq!(reflected.r, 0.19032, 1e-2);
        assert_approx_eq!(reflected.g, 0.2379, 1e-2);
        assert_approx_eq!(reflected.b, 0.14274, 1e-2);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translate(0., -1., 0.);
        w.objects.push(shape);
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );
        let xs = w.intersect(&r);
        let i = xs
            .iter()
            .find(|x| (x.t - std::f32::consts::SQRT_2).abs() < 1e-2)
            .unwrap();
        let c = w.shade(&i, 1);
        assert_approx_eq!(c.r, 0.87677, 1e-2);
        assert_approx_eq!(c.g, 0.92436, 1e-2);
        assert_approx_eq!(c.b, 0.82918, 1e-2);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.lights
            .push(point_light(point3(0., 0., 0.), color(1., 1., 1.)));

        let mut lower = plane();
        lower.material.reflective = 1.;
        lower.transform = translate(0., -1., 0.);
        w.objects.push(lower);

        let mut upper = plane();
        upper.material.reflective = 1.;
        upper.transform = translate(0., 1., 0.);
        w.objects.push(upper);

        let r = ray(point3(0., 0., 0.), vector3(0., 1., 0.));
        let c = w.color_at(&r);
        assert_eq!(c.r, 1.);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.transform = translate(0., -1., 0.);
        w.objects.push(shape);
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );
        let xs = w.intersect(&r);
        let i = xs
            .iter()
            .find(|x| (x.t - std::f32::consts::SQRT_2).abs() < 1e-2)
            .unwrap();
        let c = w.reflected_color(&i, 0);
        assert_eq!(c, color(0., 0., 0.));
    }

    #[bench]
    fn bench_intersect_a_world_with_a_ray(bencher: &mut Bencher) {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        bencher.iter(|| w.intersect(&r));
    }

    #[bench]
    fn bench_shading_an_intersection(bencher: &mut Bencher) {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs = w.intersect(&r);
        let i = xs.iter().find(|x| x.t == 4.).unwrap();
        bencher.iter(|| w.shade(&i, 1));
    }
}
