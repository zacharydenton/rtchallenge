extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::scene::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut camera = Camera::new(1000, 500, std::f32::consts::FRAC_PI_3);
    camera.set_transform(Transform::look_at(
        point3(0., 1.5, -5.),
        point3(0., 1., 0.),
        vector3(0., 1., 0.),
    ));

    let mut scene = Scene::new();
    scene.add_light(Light::new(point3(-10., 10., -10.), Color::new(1., 1., 1.)));

    let floor_material = Material::new().color(Color::new(1., 0.9, 0.9)).specular(0.);
    scene.add_object(
        Object::new()
            .transform(Transform::new().scale(10., 0.01, 10.))
            .material(floor_material),
    );
    scene.add_object(
        Object::new()
            .transform(
                Transform::new()
                    .translate(0., 0., 5.)
                    .rotate_y(-std::f32::consts::FRAC_PI_4)
                    .rotate_x(std::f32::consts::FRAC_PI_2)
                    .scale(10., 0.01, 10.),
            )
            .material(floor_material),
    );
    scene.add_object(
        Object::new()
            .transform(
                Transform::new()
                    .translate(0., 0., 5.)
                    .rotate_y(std::f32::consts::FRAC_PI_4)
                    .rotate_x(std::f32::consts::FRAC_PI_2)
                    .scale(10., 0.01, 10.),
            )
            .material(floor_material),
    );

    scene.add_object(
        Object::new()
            .transform(Transform::new().scale(1., 0.5, 2.).translate(-0.5, 1., 0.5))
            .material(
                Material::new()
                    .color(Color::new(0.1, 1., 0.5))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );
    scene.add_object(
        Object::new()
            .transform(
                Transform::new()
                    .shear(-1., 1., 0., 0., 0., 0.)
                    .translate(1.2, 1.5, 0.)
                    .scale(1.4, 0.5, 0.5),
            )
            .material(
                Material::new()
                    .color(Color::new(0.5, 1., 0.1))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );
    scene.add_object(
        Object::new()
            .transform(
                Transform::new()
                    .translate(-1.7, 0.33, -0.75)
                    .scale(0.33, 0.7, 0.33),
            )
            .material(
                Material::new()
                    .color(Color::new(1., 0.8, 0.1))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let canvas = camera.render(scene);
    print!("{}", canvas_to_ppm(canvas));
}
