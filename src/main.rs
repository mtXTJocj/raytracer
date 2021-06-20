use raytracer::{
    camera::Camera, checkers_pattern::CheckersPattern, color::Color,
    cone::Cone, cylinder::Cylinder, light::Light, node::Node, pattern::Pattern,
    plane::Plane, point3d::Point3D, shape::Shape, transform::Transform,
    vector3d::Vector3D, world::World, FLOAT,
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
    floor.material_mut().color = Color::new(1.0, 0.9, 0.9);
    floor.material_mut().specular = 0.0;
    let mut pattern =
        Box::new(CheckersPattern::new(Color::WHITE, Color::BLACK));
    *pattern.transform_mut() = Transform::scaling(0.8, 0.8, 0.8);
    *floor.material_mut().pattern_mut() = Some(pattern);
    let mut floor = Node::new(floor);
    floor.set_transform(Transform::translation(0.0, 0.0, 5.0));

    let mut cyl0 = Box::new(Cylinder::new());
    *cyl0.minimum_mut() = -1.0;
    *cyl0.maximum_mut() = 1.0;
    *cyl0.closed_mut() = true;
    cyl0.material_mut().color = Color::new(0.1, 0.7, 0.7);
    cyl0.material_mut().reflective = 1.0;
    cyl0.material_mut().transparency = 0.7;
    cyl0.material_mut().refractive_index = 0.9;
    let mut cyl0 = Node::new(cyl0);
    cyl0.set_transform(
        &(&(&Transform::translation(-1.0, 1.0, 0.5)
            * &Transform::rotation_y(std::f32::consts::FRAC_PI_4 as FLOAT))
            * &Transform::rotation_x(std::f32::consts::FRAC_PI_4 as FLOAT))
            * &Transform::scaling(0.5, 1.0, 0.5),
    );

    let mut cone0 = Box::new(Cone::new());
    *cone0.minimum_mut() = -1.2;
    *cone0.maximum_mut() = 0.4;
    *cone0.closed_mut() = true;
    cone0.material_mut().color = Color::new(0.7, 0.1, 0.7);
    cone0.material_mut().reflective = 1.0;
    cone0.material_mut().transparency = 0.7;
    cone0.material_mut().refractive_index = 0.9;
    let mut cone0 = Node::new(cone0);
    cone0.set_transform(
        &(&(&Transform::translation(1.0, 1.0, 0.5)
            * &Transform::rotation_y(std::f32::consts::FRAC_PI_4 as FLOAT))
            * &Transform::rotation_x(std::f32::consts::FRAC_PI_4 as FLOAT))
            * &Transform::scaling(0.5, 1.0, 0.5),
    );

    world.add_node(floor);
    world.add_node(cyl0);
    world.add_node(cone0);
    world.add_light(Light::new(
        Point3D::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera =
        Camera::new(600, 300, std::f32::consts::FRAC_PI_3 as FLOAT);
    *camera.transform_mut() = Transform::view_transform(
        &Point3D::new(0.0, 1.5, -5.0),
        &Point3D::new(0.0, 1.0, 0.0),
        &Vector3D::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    canvas.to_ppm(&mut writer).expect("write failed");
}
