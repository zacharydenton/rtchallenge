use crate::color::*;
use crate::light::*;
use crate::object::*;
use crate::ray::*;
use crate::transform::*;
use crate::tuple::*;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

pub fn world() -> World {
    World {
        objects: vec![],
        lights: vec![],
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
    pub fn shade(&self, intersection: &Intersection) -> Color {
        self.lights
            .iter()
            .map(|light| {
                intersection.object.material.lighting(
                    light,
                    &intersection.point.unwrap(),
                    &intersection.eyev.unwrap(),
                    &intersection.normalv.unwrap(),
                )
            })
            .fold(color(0., 0., 0.), |acc, x| acc + x)
    }

    /// Intersects the ray with the world and returns the color at the resulting
    /// intersection.
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);

        if intersections.len() == 0 {
            return color(0., 0., 0.);
        }

        self.shade(&intersections[0])
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
        let c = w.shade(&i);

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
        let c = w.shade(&i);

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
        bencher.iter(|| w.shade(&i));
    }
}
