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
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect::<Intersections>();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[bench]
    fn bench_intersect_a_world_with_a_ray(bencher: &mut Bencher) {
        let w = default_world();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        bencher.iter(|| w.intersect(&r));
    }
}
