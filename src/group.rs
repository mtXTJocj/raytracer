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

        xs.sort_unstable_by(|i1, i2| {
            if i1.t < i2.t {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        xs
    }

    fn local_normal_at(&self, p: &Point3D) -> Vector3D {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{sphere::Sphere, transform::Transform},
        *,
    };

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();

        assert_eq!(0, g.children.len());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut p = Node::new(Box::new(Group::new()));
        let mut c = Node::new(Box::new(Group::new()));
        let child_ptr = &*c;

        p.add_child(c);
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
        let mut g = Node::new(Box::new(Group::new()));
        let s1 = Node::new(Box::new(Sphere::new()));
        let mut s2 = Node::new(Box::new(Sphere::new()));
        s2.set_transform(Transform::translation(0.0, 0.0, -3.0));
        let mut s3 = Node::new(Box::new(Sphere::new()));
        s3.set_transform(Transform::translation(5.0, 0.0, 0.0));

        let s1_ptr = &*s1 as *const Node;
        let s2_ptr = &*s2 as *const Node;
        g.add_child(s1);
        g.add_child(s2);
        g.add_child(s3);

        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = g.intersect(&r);
        assert_eq!(4, xs.len());

        assert!(std::ptr::eq(s2_ptr, xs[0].object));
        assert!(std::ptr::eq(s2_ptr, xs[1].object));
        assert!(std::ptr::eq(s1_ptr, xs[2].object));
        assert!(std::ptr::eq(s1_ptr, xs[3].object));
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Node::new(Box::new(Group::new()));
        g.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let mut s = Node::new(Box::new(Sphere::new()));
        s.set_transform(Transform::translation(5.0, 0.0, 0.0));
        g.add_child(s);

        let r = Ray::new(
            Point3D::new(10.0, 0.0, -10.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = g.intersect(&r);

        assert_eq!(2, xs.len());
    }
}
