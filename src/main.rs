use raytracer::{
    camera::Camera, checkers_pattern::CheckersPattern, color::Color,
    cylinder::Cylinder, group::Group, light::Light, node::Node,
    pattern::Pattern, plane::Plane, point3d::Point3D, shape::Shape,
    sphere::Sphere, transform::Transform, vector3d::Vector3D, world::World,
    FLOAT,
};

use std::{
    boxed::Box,
    env,
    fs::File,
    io::{stdout, BufWriter, Write},
};

fn hexagon_corner() -> Box<Node> {
    let mut corner = Node::new(Box::new(Sphere::new()));
    corner.set_transform(
        &Transform::translation(0.0, 0.0, -1.0)
            * &Transform::scaling(0.25, 0.25, 0.25),
    );

    corner
}

fn hexagon_edge() -> Box<Node> {
    let mut cyl = Cylinder::new();
    *cyl.minimum_mut() = 0.0;
    *cyl.maximum_mut() = 1.0;
    let mut edge = Node::new(Box::new(cyl));
    edge.set_transform(
        &(&(&Transform::translation(0.0, 0.0, -1.0)
            * &Transform::rotation_y(-std::f64::consts::FRAC_PI_6 as FLOAT))
            * &Transform::rotation_z(-std::f64::consts::FRAC_PI_2 as FLOAT))
            * &Transform::scaling(0.25, 1.0, 0.25),
    );

    edge
}

fn hexagon_side() -> Box<Node> {
    let mut side = Node::new(Box::new(Group::new()));

    let corner = hexagon_corner();
    let edge = hexagon_edge();
    side.add_child(corner);
    side.add_child(edge);

    side
}

fn hexagon() -> Box<Node> {
    let mut hex = Node::new(Box::new(Group::new()));

    for n in 0..6 {
        let mut side = hexagon_side();
        side.set_transform(Transform::rotation_y(
            n as FLOAT * std::f64::consts::FRAC_PI_3 as FLOAT,
        ));
        hex.add_child(side);
    }

    hex
}

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
    world.add_node(floor);

    let mut hex = hexagon();
    hex.set_transform(Transform::translation(0.0, 0.5, 0.0));
    world.add_node(hex);

    world.add_light(Light::new(
        Point3D::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera =
        Camera::new(600, 300, std::f32::consts::FRAC_PI_3 as FLOAT);
    *camera.transform_mut() = Transform::view_transform(
        &Point3D::new(0.0, 3.5, -5.0),
        &Point3D::new(0.0, 0.0, 0.0),
        &Vector3D::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    canvas.to_ppm(&mut writer).expect("write failed");
}
