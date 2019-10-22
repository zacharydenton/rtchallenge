use crate::color::*;
use crate::geometry::*;
use crate::intersection::*;
use crate::light::*;
use crate::material::*;
use crate::object::*;
use crate::ray::*;
use crate::transform::*;
use crate::tuple::*;

pub struct Scene {
    lights: Vec<Light>,
    transforms: Vec<Transform>,
    materials: Vec<Material>,
    geometrys: Vec<Geometry>,
    max_depth: usize,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: vec![],
            transforms: vec![],
            materials: vec![],
            geometrys: vec![],
            max_depth: 5,
        }
    }

    /// Intersects the ray with the world and returns the color at the resulting
    /// intersection.
    pub fn color_at(&self, world_ray: Ray) -> Color {
        self.color_at_remaining(world_ray, self.max_depth).clamp()
    }

    /// Intersects the ray with the world and returns the color at the resulting
    /// intersection (with specified remaining depth).
    fn color_at_remaining(&self, world_ray: Ray, remaining: usize) -> Color {
        if remaining == 0 {
            return Color {
                r: 0.,
                g: 0.,
                b: 0.,
            };
        }

        if let Some(intersection) = self.nearest_intersection(world_ray) {
            let transform = self.transforms[intersection.object_id];
            let material = self.materials[intersection.object_id];
            let geometry = self.geometrys[intersection.object_id];

            // Compute the surface normal.
            let world_point = world_ray.position(intersection.t);
            let eye_vector = -world_ray.direction;
            let world_normal = world_normal_at(transform, geometry, world_point, eye_vector);

            // Compute surface color.
            let over_point = world_point + world_normal * 1e-3;
            let under_point = world_point - world_normal * 1e-3;
            let light = self.lights[0]; // TODO: Support more than one light.
            let in_shadow = self.is_shadowed(over_point, light);
            let surface_color = material.lighting(
                transform,
                light,
                world_point,
                eye_vector,
                world_normal,
                in_shadow,
            );

            // Compute reflect color.
            let reflect_color = if material.reflective > 0. && remaining > 0 {
                let reflect_vector = world_ray.direction.reflect(world_normal);
                let reflect_ray = ray(over_point, reflect_vector);
                self.color_at_remaining(reflect_ray, remaining - 1) * material.reflective
            } else {
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                }
            };

            // Compute refract color.
            let (n1, n2) = if material.transparency > 0. {
                self.refractive_indexes(world_ray, intersection)
            } else {
                // Skip computation if the values aren't needed.
                (1.0, 1.0)
            };
            let refract_color = if material.transparency > 0. && remaining > 0 {
                let n_ratio = n1 / n2;
                let cos_i = eye_vector.dot(world_normal);
                let sin2_t = n_ratio * n_ratio * (1. - cos_i * cos_i);

                if sin2_t > 1. {
                    // Total internal reflection.
                    Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    }
                } else {
                    let cos_t = (1. - sin2_t).sqrt();
                    let direction = world_normal * (n_ratio * cos_i - cos_t) - eye_vector * n_ratio;
                    let refract_ray = ray(under_point, direction);
                    let refract_color = self.color_at_remaining(refract_ray, remaining - 1);
                    refract_color * material.transparency
                }
            } else {
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                }
            };

            if material.reflective > 0. && material.transparency > 0. {
                // Apply Fresnel effect.
                let reflectance = schlick(eye_vector, world_normal, n1, n2);
                surface_color + reflect_color * reflectance + refract_color * (1. - reflectance)
            } else {
                surface_color + reflect_color + refract_color
            }
        } else {
            Color {
                r: 0.,
                g: 0.,
                b: 0.,
            }
        }
    }

    /// Returns an iterator of all intersections between the ray and the scene.
    pub fn intersections(&self, world_ray: Ray) -> impl Iterator<Item = Intersection> + '_ {
        let local_rays = self
            .transforms
            .iter()
            .map(move |transform| world_ray.transform(transform.world_to_local));
        local_rays.zip(self.geometrys.iter()).enumerate().flat_map(
            |(object_id, (local_ray, geometry))| {
                geometry
                    .intersect(local_ray)
                    .map(move |t| Intersection { t, object_id })
            },
        )
    }

    /// Returns the nearest intersection (if any).
    pub fn nearest_intersection(&self, world_ray: Ray) -> Option<Intersection> {
        self.intersections(world_ray)
            .filter(|intersection| intersection.t >= 0.)
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }

    /// Whether the given point is considered to be in shadow.
    pub fn is_shadowed(&self, point: Tuple4, light: Light) -> bool {
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        if let Some(intersection) = self.nearest_intersection(ray(point, direction)) {
            intersection.t < distance
        } else {
            false
        }
    }

    /// Returns the indexes of refraction of the materials on either side of a
    /// ray-object intersection, with n1 belonging to the material being
    /// exited, and n2 belonging to the material being entered.
    pub fn refractive_indexes(&self, world_ray: Ray, intersection: Intersection) -> (f32, f32) {
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        let mut containers: Vec<ObjectId> = vec![];
        let mut all_intersections: Vec<Intersection> = self.intersections(world_ray).collect();
        all_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        for i in all_intersections {
            if i == intersection {
                if containers.len() == 0 {
                    n1 = 1.0;
                } else {
                    n1 = self.materials[*containers.last().unwrap()].refractive_index;
                }
            }

            if containers.contains(&i.object_id) {
                containers.retain(|o| o != &i.object_id);
            } else {
                containers.push(i.object_id);
            }

            if i == intersection {
                if containers.len() == 0 {
                    n2 = 1.0;
                } else {
                    n2 = self.materials[*containers.last().unwrap()].refractive_index;
                }
                break;
            }
        }

        (n1, n2)
    }

    /// Adds the light to the scene.
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Adds the object to the scene, returning its ID.
    pub fn add_object(&mut self, object: Object) -> ObjectId {
        let object_id = self.transforms.len();

        self.transforms.push(object.transform);
        self.materials.push(object.material);
        self.geometrys.push(object.geometry);

        debug_assert!(
            (self.transforms.len() == self.materials.len())
                == (self.geometrys.len() == object_id + 1)
        );

        object_id
    }
}

/// Computes the Schlick approximation for the given intersection.
pub fn schlick(eyev: Tuple4, normalv: Tuple4, n1: f32, n2: f32) -> f32 {
    let mut cos = eyev.dot(normalv);

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

/// Computes the world normal vector at the given point.
pub fn world_normal_at(
    transform: Transform,
    geometry: Geometry,
    world_point: Tuple4,
    eye_vector: Tuple4,
) -> Tuple4 {
    let local_point = transform.world_to_local * world_point;
    let local_normal = geometry.normal_at(local_point);
    let mut world_normal = transform.world_to_local.transpose() * local_normal;
    world_normal.w = 0.;
    world_normal = world_normal.normalize();

    if world_normal.dot(eye_vector) < 0. {
        // The ray originates inside the object.
        world_normal = -world_normal;
    }

    world_normal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::*;
    use assert_approx_eq::assert_approx_eq;
    use test::Bencher;

    fn default_scene() -> Scene {
        let mut scene = Scene::new();
        scene.add_light(Light::new(point3(-10., 10., -10.), Color::new(1., 1., 1.)));
        scene.add_object(
            Object::new().geometry(Geometry::sphere()).material(
                Material::new()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2),
            ),
        );
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(0.5, 0.5, 0.5)),
        );
        scene
    }

    #[test]
    fn creating_a_scene() {
        let w = Scene::new();
        assert_eq!(w.transforms.len(), 0);
        assert_eq!(w.geometrys.len(), 0);
        assert_eq!(w.materials.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_scene() {
        let scene = default_scene();
        assert_eq!(scene.transforms.len(), 2);
        assert_eq!(scene.geometrys.len(), 2);
        assert_eq!(scene.materials.len(), 2);
        assert_eq!(scene.lights.len(), 1);
    }

    #[test]
    fn intersect_a_scene_with_a_ray() {
        let scene = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let mut xs: Vec<Intersection> = scene.intersections(r).collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(scene.nearest_intersection(r).unwrap(), xs[0]);
        assert_eq!(xs[0].object_id, 0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[1].object_id, 1);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[2].object_id, 1);
        assert_eq!(xs[3].t, 6.0);
        assert_eq!(xs[3].object_id, 0);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let scene = default_scene();
        let p = point3(0., 10., 0.);
        assert_eq!(scene.is_shadowed(p, scene.lights[0]), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let scene = default_scene();
        let p = point3(10., -10., 10.);
        assert_eq!(scene.is_shadowed(p, scene.lights[0]), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let scene = default_scene();
        let p = point3(-20., 20., -20.);
        assert_eq!(scene.is_shadowed(p, scene.lights[0]), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let scene = default_scene();
        let p = point3(-2., 2., -2.);
        assert_eq!(scene.is_shadowed(p, scene.lights[0]), false);
    }

    #[test]
    fn shading_an_intersection() {
        let scene = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.38066, 1e-5);
        assert_approx_eq!(c.g, 0.47583, 1e-5);
        assert_approx_eq!(c.b, 0.2855, 1e-5);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut scene = default_scene();
        scene.lights = vec![Light::new(point3(0., 0.25, 0.), Color::new(1., 1., 1.))];
        let r = ray(point3(0., 0., 0.), vector3(0., 0., 1.));
        let i = scene.nearest_intersection(r).unwrap();
        let c = scene.color_at(r);

        assert_eq!(i.t, 0.5);
        assert_eq!(i.object_id, 1);
        assert_approx_eq!(c.r, 0.90498, 1e-5);
        assert_approx_eq!(c.g, 0.90498, 1e-5);
        assert_approx_eq!(c.b, 0.90498, 1e-5);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let scene = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 1., 0.));
        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.0, 1e-5);
        assert_approx_eq!(c.g, 0.0, 1e-5);
        assert_approx_eq!(c.b, 0.0, 1e-5);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let scene = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.38066, 1e-5);
        assert_approx_eq!(c.g, 0.47583, 1e-5);
        assert_approx_eq!(c.b, 0.2855, 1e-5);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut scene = Scene::new();
        scene.add_light(Light::new(point3(-10., 10., -10.), Color::new(1., 1., 1.)));
        scene.add_object(
            Object::new().geometry(Geometry::sphere()).material(
                Material::new()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .ambient(1.0)
                    .diffuse(0.7)
                    .specular(0.2),
            ),
        );
        scene.add_object(
            Object::new()
                .material(Material::new().ambient(1.0))
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(0.5, 0.5, 0.5)),
        );

        let r = ray(point3(0., 0., 0.75), vector3(0., 0., -1.));
        let c = scene.color_at(r);

        assert_eq!(c, scene.materials[1].color);
    }

    #[test]
    fn shade_is_given_an_intersection_in_shadow() {
        let mut scene = Scene::new();
        scene.add_light(Light::new(point3(0., 0., -10.), Color::new(1., 1., 1.)));
        scene.add_object(Object::new().geometry(Geometry::sphere()));
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(0., 0., 10.)),
        );
        let r = ray(point3(0., 0., 5.), vector3(0., 0., 1.));

        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.1, 1e-5);
        assert_approx_eq!(c.g, 0.1, 1e-5);
        assert_approx_eq!(c.b, 0.1, 1e-5);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut scene = default_scene();
        scene.add_object(
            Object::new()
                .geometry(Geometry::plane())
                .material(
                    Material::new()
                        .reflective(0.5)
                        .color(Color::new(0., 0., 0.))
                        .diffuse(0.)
                        .specular(0.),
                )
                .transform(Transform::new().translate(0., -1., 0.)),
        );
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );

        let c = scene.color_at(r);
        assert_approx_eq!(c.r, 0.19032, 1e-2);
        assert_approx_eq!(c.g, 0.2379, 1e-2);
        assert_approx_eq!(c.b, 0.14274, 1e-2);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut scene = default_scene();
        scene.add_object(
            Object::new()
                .geometry(Geometry::plane())
                .material(Material::new().reflective(0.5))
                .transform(Transform::new().translate(0., -1., 0.)),
        );
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );

        let c = scene.color_at(r);
        assert_approx_eq!(c.r, 0.87677, 1e-2);
        assert_approx_eq!(c.g, 0.92436, 1e-2);
        assert_approx_eq!(c.b, 0.82918, 1e-2);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut scene = Scene::new();
        scene.add_light(Light::new(point3(0., 0., 0.), Color::new(1., 1., 1.)));

        let lower = Object::new()
            .material(Material::new().reflective(1.))
            .transform(Transform::new().translate(0., -1., 0.));;
        scene.add_object(lower);

        let upper = Object::new()
            .material(Material::new().reflective(1.))
            .transform(Transform::new().translate(0., 1., 0.));;
        scene.add_object(upper);

        let r = ray(point3(0., 0., 0.), vector3(0., 1., 0.));
        let c = scene.color_at(r);

        // Test that the color_at function terminates with infinitely recursive rays.
        assert_eq!(c.r, 1.);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut scene = default_scene();
        scene.add_object(
            Object::new()
                .material(Material::new().reflective(0.5))
                .transform(Transform::new().translate(0., -1., 0.)),
        );
        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );

        let c = scene.color_at_remaining(r, 0);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut scene = default_scene();
        let mut material = scene.materials.first_mut().unwrap();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let c = scene.color_at_remaining(r, 0);
        assert_eq!(c, Color::new(0., 0., 0.,));
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut scene = default_scene();
        let mut material = scene.materials.first_mut().unwrap();
        material.color = Color::new(0., 0., 0.);
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        let r = ray(
            point3(0., 0., std::f32::consts::SQRT_2 * 0.5),
            vector3(0., 1., 0.),
        );
        let c = scene.color_at(r);
        assert_eq!(c, Color::new(0., 0., 0.,));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut scene = default_scene();
        let a = scene.materials.first_mut().unwrap();
        a.ambient = 1.0;
        a.pattern = Some(test_pattern());
        let b = scene.materials.last_mut().unwrap();
        b.ambient = 0.;
        b.transparency = 1.0;
        b.refractive_index = 1.5;
        let r = ray(point3(0., 0., 0.1), vector3(0., 1., 0.));
        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.0, 1e-2);
        assert_approx_eq!(c.g, 0.99888, 1e-2);
        assert_approx_eq!(c.b, 0.04725, 1e-2);
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut scene = default_scene();
        scene.add_object(
            Object::new()
                .geometry(Geometry::plane())
                .transform(Transform::new().translate(0., -1., 0.))
                .material(Material::new().transparency(0.5).refractive_index(1.5)),
        );
        scene.add_object(
            Object::new()
                .transform(Transform::new().translate(0., -3.5, -0.5))
                .material(Material::new().color(Color::new(1., 0., 0.)).ambient(0.5)),
        );

        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );

        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.93642, 1e-2);
        assert_approx_eq!(c.g, 0.68642, 1e-2);
        assert_approx_eq!(c.b, 0.68642, 1e-2);
    }

    #[test]
    fn shade_hit_with_a_reflective_and_transparent_material() {
        let mut scene = default_scene();

        let floor = Object::new()
            .geometry(Geometry::plane())
            .transform(Transform::new().translate(0., -1., 0.))
            .material(
                Material::new()
                    .reflective(0.5)
                    .transparency(0.5)
                    .refractive_index(1.5),
            );
        scene.add_object(floor);

        let ball = Object::new()
            .transform(Transform::new().translate(0., -3.5, -0.5))
            .material(Material::new().color(Color::new(1., 0., 0.)).ambient(0.5));
        scene.add_object(ball);

        let r = ray(
            point3(0., 0., -3.),
            vector3(
                0.,
                -std::f32::consts::SQRT_2 * 0.5,
                std::f32::consts::SQRT_2 * 0.5,
            ),
        );
        let c = scene.color_at(r);

        assert_approx_eq!(c.r, 0.93391, 1e-2);
        assert_approx_eq!(c.g, 0.69643, 1e-2);
        assert_approx_eq!(c.b, 0.69243, 1e-2);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let mut scene = Scene::new();
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(2., 2., 2.)),
        );
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs: Vec<Intersection> = scene.intersections(r).collect();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let mut scene = Scene::new();
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(5., 0., 0.)),
        );
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        let xs: Vec<Intersection> = scene.intersections(r).collect();
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let transform = Transform::new();
        let geometry = Geometry::sphere();
        let root3over3 = (3 as f32).sqrt() / 3.;
        let world_point = point3(root3over3, root3over3, root3over3);
        let eye_vector = world_point - point3(0., 0., 0.);
        let n = world_normal_at(transform, geometry, world_point, eye_vector);
        let normalized = n.normalize();
        assert_approx_eq!(n.x, normalized.x);
        assert_approx_eq!(n.y, normalized.y);
        assert_approx_eq!(n.z, normalized.z);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let transform = Transform::new().translate(0., 1., 0.);
        let geometry = Geometry::sphere();
        let world_point = point3(0., 1.70711, -0.70711);
        let eye_vector = world_point - point3(0., 0., 0.);
        let n = world_normal_at(transform, geometry, world_point, eye_vector);
        assert_approx_eq!(n.x, 0., 1e-5);
        assert_approx_eq!(n.y, 0.70711, 1e-5);
        assert_approx_eq!(n.z, -0.70711, 1e-5);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let transform = Transform::new()
            .scale(1., 0.5, 1.)
            .rotate_z(std::f32::consts::PI / 5.);
        let geometry = Geometry::sphere();
        let world_point = point3(
            0.,
            2. * std::f32::consts::FRAC_1_SQRT_2,
            -2. * std::f32::consts::FRAC_1_SQRT_2,
        );
        let eye_vector = world_point - point3(0., 0., 0.);
        let n = world_normal_at(transform, geometry, world_point, eye_vector);
        assert_approx_eq!(n.x, 0., 1e-5);
        assert_approx_eq!(n.y, 0.97014, 1e-5);
        assert_approx_eq!(n.z, -0.24254, 1e-5);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut scene = Scene::new();

        let a = scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(2., 2., 2.))
                .material(Material::new().transparency(1.).refractive_index(1.5)),
        );
        let b = scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(0., 0., -0.25))
                .material(Material::new().transparency(1.).refractive_index(2.0)),
        );
        let c = scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(0., 0., 0.25))
                .material(Material::new().transparency(1.).refractive_index(2.5)),
        );

        let r = ray(point3(0., 0., -4.), vector3(0., 0., 1.));
        let expected_intersections = vec![
            Intersection {
                t: 2.,
                object_id: a,
            },
            Intersection {
                t: 2.75,
                object_id: b,
            },
            Intersection {
                t: 3.25,
                object_id: c,
            },
            Intersection {
                t: 4.75,
                object_id: b,
            },
            Intersection {
                t: 5.25,
                object_id: c,
            },
            Intersection {
                t: 6.,
                object_id: a,
            },
        ];
        let mut actual_intersections: Vec<Intersection> = scene.intersections(r).collect();
        actual_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        assert_eq!(actual_intersections, expected_intersections);

        let expected_refractive_indexes = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (&intersection, refractive_indexes) in
            actual_intersections.iter().zip(expected_refractive_indexes)
        {
            assert_eq!(
                scene.refractive_indexes(r, intersection),
                refractive_indexes
            );
        }
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let mut scene = Scene::new();
        let transform = Transform::new();
        let geometry = Geometry::sphere();
        scene.add_object(
            Object::new()
                .geometry(geometry)
                .transform(transform)
                .material(Material::new().transparency(1.).refractive_index(1.5)),
        );
        let r = ray(
            point3(0., 0., std::f32::consts::SQRT_2 * 0.5),
            vector3(0., 1., 0.),
        );
        let intersection = scene.nearest_intersection(r).unwrap();
        assert_eq!(intersection.t, std::f32::consts::SQRT_2 * 0.5);
        let world_point = r.position(intersection.t);
        let eyev = -r.direction;
        let normalv = world_normal_at(transform, geometry, world_point, eyev);
        assert_approx_eq!(normalv.x, 0.);
        assert_approx_eq!(normalv.y, -std::f32::consts::SQRT_2 * 0.5);
        assert_approx_eq!(normalv.z, -std::f32::consts::SQRT_2 * 0.5);
        let (n1, n2) = scene.refractive_indexes(r, intersection);
        assert_eq!(n1, 1.5);
        assert_eq!(n2, 1.0);
        let reflectance = schlick(eyev, normalv, n1, n2);
        assert_approx_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let mut scene = Scene::new();
        let transform = Transform::new();
        let geometry = Geometry::sphere();
        scene.add_object(
            Object::new()
                .geometry(geometry)
                .transform(transform)
                .material(Material::new().transparency(1.).refractive_index(1.5)),
        );
        let r = ray(point3(0., 0., 0.), vector3(0., 1., 0.));
        let intersection = scene.nearest_intersection(r).unwrap();
        let (n1, n2) = scene.refractive_indexes(r, intersection);
        let world_point = r.position(intersection.t);
        let eyev = -r.direction;
        let normalv = world_normal_at(transform, geometry, world_point, eyev);
        let reflectance = schlick(eyev, normalv, n1, n2);
        assert_approx_eq!(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let mut scene = Scene::new();
        let transform = Transform::new();
        let geometry = Geometry::sphere();
        scene.add_object(
            Object::new()
                .geometry(geometry)
                .transform(transform)
                .material(Material::new().transparency(1.).refractive_index(1.5)),
        );
        let r = ray(point3(0., 0.99, -2.), vector3(0., 0., 1.));
        let intersection = scene.nearest_intersection(r).unwrap();
        let (n1, n2) = scene.refractive_indexes(r, intersection);
        let world_point = r.position(intersection.t);
        let eyev = -r.direction;
        let normalv = world_normal_at(transform, geometry, world_point, eyev);
        let reflectance = schlick(eyev, normalv, n1, n2);
        assert_approx_eq!(reflectance, 0.48873, 1e-3);
    }

    #[bench]
    fn bench_intersect_a_scene_with_a_ray(bencher: &mut Bencher) {
        let w = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        bencher.iter(|| w.nearest_intersection(r).unwrap());
    }

    #[bench]
    fn bench_shading_an_intersection(bencher: &mut Bencher) {
        let scene = default_scene();
        let r = ray(point3(0., 0., -5.), vector3(0., 0., 1.));
        bencher.iter(|| scene.color_at(r));
    }

    #[bench]
    fn bench_finding_n1_and_n2(bencher: &mut Bencher) {
        let mut scene = Scene::new();

        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(2., 2., 2.))
                .material(Material::new().transparency(1.).refractive_index(1.5)),
        );
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(0., 0., -0.25))
                .material(Material::new().transparency(1.).refractive_index(2.0)),
        );
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().translate(0., 0., 0.25))
                .material(Material::new().transparency(1.).refractive_index(2.5)),
        );

        let r = ray(point3(0., 0., -4.), vector3(0., 0., 1.));
        let intersection = scene.nearest_intersection(r).unwrap();

        bencher.iter(|| {
            let (_n1, _n2) = scene.refractive_indexes(r, intersection);
        });
    }
}
