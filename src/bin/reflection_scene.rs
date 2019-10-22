extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::geometry::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::scene::*;
use rtchallenge::texture::*;
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

    let mut floor_texture =
        Texture::checkerboard_2d(Color::new(1., 1., 1.), Color::new(0., 0., 0.));
    floor_texture.transform = Transform::new().scale(0.05, 0.05, 0.05);
    let floor_material = Material::new().texture(floor_texture).specular(0.);

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
            .transform(Transform::new().scale(10., 0.01, 10.))
            .material(floor_material),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::cube())
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
            .geometry(Geometry::cube())
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
            .geometry(Geometry::sphere())
            .transform(Transform::new().translate(0.5, 1., -1.1))
            .material(
                Material::new()
                    .color(Color::new(0.5, 0.5, 0.5))
                    .diffuse(0.3)
                    .specular(1.0)
                    .shininess(400)
                    .reflective(0.9)
                    .transparency(0.9)
                    .refractive_index(1.5),
            ),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .translate(1.2, 1.5, -0.75)
                    .scale(0.5, 0.5, 0.5),
            )
            .material(
                Material::new()
                    .color(Color::new(0.5, 1., 0.1))
                    .diffuse(0.7)
                    .specular(0.3)
                    .reflective(0.1),
            ),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .translate(-0.9, 0.33, -1.25)
                    .scale(0.25, 0.25, 0.25),
            )
            .material(
                Material::new()
                    .color(Color::new(0.5, 0., 0.))
                    .ambient(0.7)
                    .diffuse(0.9)
                    .shininess(300)
                    .specular(0.9)
                    .reflective(1.0)
                    .transparency(0.05)
                    .refractive_index(1.5),
            ),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .translate(-0.3, 1.73, -1.95)
                    .scale(0.35, 0.35, 0.35),
            )
            .material(
                Material::new()
                    .color(Color::new(0.2, 0.1, 1.0))
                    .diffuse(0.7)
                    .specular(0.3)
                    .reflective(0.2),
            ),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .translate(0.5, 0.73, 0.15)
                    .scale(0.35, 0.35, 0.35),
            )
            .material(
                Material::new()
                    .color(Color::new(0.2, 0.1, 1.0))
                    .diffuse(0.7)
                    .specular(0.3)
                    .reflective(0.2),
            ),
    );

    let canvas = camera.render(scene, 10);
    print!("{}", canvas_to_ppm(canvas));
}
