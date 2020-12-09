use super::{
    intersection::Intersection, light::Light, ray::Ray, sphere::Sphere,
    vector3d::Vector3D,
};

#[derive(Debug)]
pub struct World {
    lights: Vec<Light>,
    shapes: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            lights: vec![],
            shapes: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_shape(&mut self, sphere: Sphere) {
        self.shapes.push(sphere);
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for shape in &self.shapes {
            let mut xs = shape.intersect(ray);
            intersections.append(&mut xs);
        }

        intersections.sort_unstable_by(|i1, i2| {
            if i1.t < i2.t {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            approx_eq, color::Color, material::Material, point3d::Point3D,
            transform::Transform,
        },
        *,
    };

    fn default_world() -> World {
        let mut w = World::new();

        let light = Light::new(
            Point3D::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        w.add_light(light);

        let mut sphere = Sphere::new();
        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        *sphere.material_mut() = material;
        w.add_shape(sphere);

        let mut sphere = Sphere::new();
        *sphere.transform_mut() = Transform::scaling(0.5, 0.5, 0.5);
        w.add_shape(sphere);
        return w;
    }

    #[test]
    fn creating_world() {
        let w = World::new();
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert!(approx_eq(4.0, xs[0].t));
        assert!(approx_eq(4.5, xs[1].t));
        assert!(approx_eq(5.5, xs[2].t));
        assert!(approx_eq(6.0, xs[3].t));
    }
}
