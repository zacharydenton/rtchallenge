extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::geometry::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::scene::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut scene = Scene::new();

    // ======================================================
    // the camera
    // ======================================================
    let mut camera = Camera::new(1000, 1000, 0.785);
    camera.set_transform(Transform::look_at(
        point3(-6., 6., -10.),
        point3(6., 0., 6.),
        vector3(-0.45, 1., 0.),
    ));

    // ======================================================
    // light sources
    // ======================================================
    scene.add_light(Light::new(point3(50., 100., -50.), Color::new(1., 1., 1.)));

    // an optional second light for additional illumination
    scene.add_light(Light::new(
        point3(-400., 50., -10.),
        Color::new(0.3, 0.3, 0.3),
    ));

    // ======================================================
    // define some constants to avoid duplication
    // ======================================================
    let white_material = || {
        Material::new()
            .color(Color::new(1., 1., 1.))
            .diffuse(0.7)
            .ambient(0.25)
            .specular(0.0)
            .reflective(0.1)
    };

    let blue_material = || {
        Material::new()
            .color(Color::new(0.537, 0.831, 0.914))
            .diffuse(0.7)
            .ambient(0.25)
            .specular(0.0)
            .reflective(0.1)
    };

    let red_material = || {
        Material::new()
            .color(Color::new(0.941, 0.322, 0.388))
            .diffuse(0.7)
            .ambient(0.25)
            .specular(0.0)
            .reflective(0.1)
    };

    let purple_material = || {
        Material::new()
            .color(Color::new(0.373, 0.404, 0.550))
            .diffuse(0.7)
            .ambient(0.25)
            .specular(0.0)
            .reflective(0.1)
    };

    let large_object = |x: f32, y: f32, z: f32| {
        Transform::new()
            .translate(x + 1., y - 1., z + 1.)
            .scale(1.75, 1.75, 1.75)
    };

    let medium_object = |x: f32, y: f32, z: f32| {
        Transform::new()
            .translate(x + 1., y - 1., z + 1.)
            .scale(1.5, 1.5, 1.5)
    };

    let small_object = |x: f32, y: f32, z: f32| Transform::new().translate(x + 1., y - 1., z + 1.);

    // ======================================================
    // a white backdrop for the scene
    // ======================================================
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .material(
                Material::new()
                    .color(Color::new(1., 1., 1.))
                    .ambient(1.)
                    .diffuse(0.)
                    .specular(0.),
            )
            .transform(
                Transform::new()
                    .translate(0., 0., 500.)
                    .rotate_x(std::f32::consts::FRAC_PI_2),
            ),
    );

    // ======================================================
    // describe the elements of the scene
    // ======================================================
    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .material(
                Material::new()
                    .color(Color::new(0.373, 0.404, 0.550))
                    .diffuse(0.2)
                    .ambient(0.0)
                    .specular(1.0)
                    .shininess(200)
                    .reflective(0.7)
                    .transparency(0.7)
                    .refractive_index(1.5),
            )
            .transform(large_object(0., 0., 0.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(medium_object(4., 0., 0.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(blue_material())
            .transform(large_object(8.5, 1.5, -0.5)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(red_material())
            .transform(large_object(0., 0., 4.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(small_object(4., 0., 4.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(purple_material())
            .transform(medium_object(7.5, 0.5, 4.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(medium_object(-0.25, 0.25, 8.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(blue_material())
            .transform(large_object(4., 1., 7.5)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(red_material())
            .transform(medium_object(10., 2., 7.5)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(small_object(8., 2., 12.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(small_object(20., 1., 9.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(blue_material())
            .transform(large_object(-0.5, -5., 0.25)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(red_material())
            .transform(large_object(4., -4., 0.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(large_object(8.5, -4., 0.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(large_object(0., -4., 4.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(purple_material())
            .transform(large_object(-0.5, -4.5, 8.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(large_object(0., -8., 4.)),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .material(white_material())
            .transform(large_object(-0.5, -8.5, 8.)),
    );

    // ======================================================
    // render the scene
    // ======================================================
    let canvas = camera.render(scene);
    print!("{}", canvas_to_ppm(canvas));
}
