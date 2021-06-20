use crate::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D,
};

#[derive(Debug)]
pub struct Group {
    children: Vec<Box<Node>>,
}

impl Group {
    pub fn new() -> Self {
        Group { children: vec![] }
    }
}

impl Shape for Group {
    fn add_child(&mut self, child: Box<Node>) {
        self.children.push(child);
    }

    fn material(&self) -> &Material {
        panic!()
    }
    fn material_mut(&mut self) -> &mut Material {
        panic!()
    }

    fn local_intersect<'a>(
        &'a self,
        r: &Ray,
        _n: &'a Node,
    ) -> Vec<Intersection<'a>> {
        let mut xs = vec![];

        for child in &self.children {
            xs.append(&mut child.intersect(r));
        }

        xs
    }

    fn local_normal_at(&self, p: &Point3D) -> Vector3D {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{node::add_child, sphere::Sphere, transform::Transform},
        *,
    };

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();

        assert_eq!(0, g.children.len());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut p = Box::new(Node::new(Box::new(Group::new())));
        let mut c = Box::new(Node::new(Box::new(Group::new())));
        let child_ptr = &*c;

        add_child(&mut p, c);
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let dummy_node = Node::new(Box::new(Group::new()));

        let g = Group::new();
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));

        let xs = g.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Box::new(Node::new(Box::new(Group::new())));
        g.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let s1 = Box::new(Node::new(Box::new(Sphere::new())));
        let mut s2 = Box::new(Node::new(Box::new(Sphere::new())));
        s2.set_transform(Transform::translation(0.0, 0.0, -3.0));
        let mut s3 = Box::new(Node::new(Box::new(Sphere::new())));
        s2.set_transform(Transform::translation(5.0, 0.0, 0.0));

        let s1_ptr = &*s1;
        let s2_ptr = &*s2;
        add_child(&mut g, s1);
        add_child(&mut g, s2);
        add_child(&mut g, s3);

        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = g.intersect(&r);
        assert_eq!(4, xs.len());

        //        assert!(std::ptr::eq(s1_ptr, xs[0].object));
        xs[1].object;
        xs[2].object;
        xs[3].object;
    }
}
