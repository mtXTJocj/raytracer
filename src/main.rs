use raytracer::{
    camera::Camera, checkers_pattern::CheckersPattern, color::Color,
    cube::Cube, gradient_pattern::GradientPattern, light::Light,
    pattern::Pattern, plane::Plane, point3d::Point3D,
    ring_pattern::RingPattern, shape::Shape, sphere::Sphere,
    stripe_pattern::StripePattern, transform::Transform, vector3d::Vector3D,
    world::World, FLOAT,
};

use std::{
    boxed::Box,
    env,
    fs::File,
    io::{stdout, BufWriter, Write},
};

fn main() {
    let mut writer: Box<dyn Write> = match env::args().nth(1) {
        None => Box::new(BufWriter::new(stdout())),
        Some(filename) => Box::new(BufWriter::new(
            File::create(filename).expect("cannot create file"),
        )),
    };

    let mut world = World::new();
    let mut floor = Box::new(Plane::new());
    *floor.transform_mut() = Transform::translation(0.0, 0.0, 5.0);
    floor.material_mut().color = Color::new(1.0, 0.9, 0.9);
    floor.material_mut().specular = 0.0;
    let mut pattern =
        Box::new(CheckersPattern::new(Color::WHITE, Color::BLACK));
    *pattern.transform_mut() = Transform::scaling(0.8, 0.8, 0.8);
    *floor.material_mut().pattern_mut() = Some(pattern);

    let mut outer = Box::new(Cube::new());
    *outer.transform_mut() = &Transform::translation(0.0, 1.0, 0.5)
        * &Transform::scaling(1.0, 1.0, 1.0);
    outer.material_mut().color = Color::new(0.1, 0.7, 0.7);
    outer.material_mut().reflective = 1.0;
    outer.material_mut().transparency = 0.7;
    outer.material_mut().refractive_index = 0.9;

    let mut inner = Box::new(Sphere::new());
    *inner.transform_mut() = Transform::translation(0.0, 1.0, 0.5);
    inner.material_mut().reflective = 1.0;
    inner.material_mut().transparency = 1.0;
    inner.material_mut().refractive_index = 0.85;

    world.add_shape(floor);
    world.add_shape(outer);
    world.add_shape(inner);
    world.add_light(Light::new(
        Point3D::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera =
        Camera::new(300, 150, std::f32::consts::FRAC_PI_3 as FLOAT);
    *camera.transform_mut() = Transform::view_transform(
        &Point3D::new(0.0, 1.5, -5.0),
        &Point3D::new(0.0, 1.0, 0.0),
        &Vector3D::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    canvas.to_ppm(&mut writer).expect("write failed");
}
