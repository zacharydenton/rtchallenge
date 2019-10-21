extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::geometry::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::pattern::*;
use rtchallenge::ppm::*;
use rtchallenge::scene::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut camera = Camera::new(1000, 500, std::f64::consts::FRAC_PI_3);
    camera.set_transform(Transform::look_at(
        point3(-4.8, 0.8, -4.8),
        point3(-2., -3., -2.),
        vector3(0., 1., 0.),
    ));

    let mut scene = Scene::new();
    scene.add_light(Light::new(point3(-5., 4.5, -2.), Color::new(1.8, 1.8, 1.8)));

    let mut floor_pattern = checkers_pattern(Color::new(1., 1., 1.), Color::new(0., 0., 0.));
    floor_pattern.transform = Transform::new().scale(0.1, 1.0, 0.1);
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(Transform::new().translate(0., 0.5, 0.).scale(10., 10., 10.))
            .material(
                Material::new()
                    .pattern(floor_pattern)
                    .diffuse(0.4)
                    .specular(0.8)
                    .shininess(30.)
                    .reflective(0.2),
            ),
    );

    let mut walls_pattern = ring_pattern(Color::new(1., 0., 0.), Color::new(0.7, 0.8, 0.9));
    walls_pattern.transform = Transform::new()
        .rotate_z(std::f64::consts::FRAC_PI_6)
        .scale(0.01, 0.01, 0.01);
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(Transform::new().scale(9., 20., 9.))
            .material(
                Material::new()
                    .pattern(walls_pattern)
                    .diffuse(0.3)
                    .specular(0.3)
                    .shininess(100.)
                    .reflective(0.1),
            ),
    );

    let mut table_pattern = stripe_pattern(Color::new(0.8, 0.5, 0.1), Color::new(0.75, 0.45, 0.08));
    table_pattern.transform = Transform::new().rotate_y(5.3).scale(0.05, 0.05, 0.05);
    let table_material = Material::new()
        .pattern(table_pattern)
        .diffuse(0.3)
        .specular(0.3)
        .shininess(100.)
        .reflective(0.02);
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(Transform::new().translate(0., -5.0, 0.).scale(3., 0.3, 2.))
            .material(table_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(
                Transform::new()
                    .translate(-2.8, -7.5, -1.8)
                    .scale(0.2, 2.7, 0.2),
            )
            .material(table_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(
                Transform::new()
                    .translate(2.8, -7.5, -1.8)
                    .scale(0.2, 2.7, 0.2),
            )
            .material(table_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(
                Transform::new()
                    .translate(-2.8, -7.5, 1.8)
                    .scale(0.2, 2.7, 0.2),
            )
            .material(table_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(
                Transform::new()
                    .translate(2.8, -7.5, 1.8)
                    .scale(0.2, 2.7, 0.2),
            )
            .material(table_material),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(
                Transform::new()
                    .translate(0.5, -4.5, 0.)
                    .rotate_y(3.)
                    .scale(0.3, 0.5, 0.3),
            )
            .material(
                Material::new()
                    .color(Color::new(0.5, 1., 0.1))
                    .diffuse(0.7)
                    .specular(0.3)
                    .reflective(0.1),
            ),
    );

    let canvas = camera.render(scene);
    print!("{}", canvas_to_ppm(canvas));
}
