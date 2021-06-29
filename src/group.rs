use crate::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D,
};

#[derive(Debug)]
pub struct Group {
    /// 子 Node
    children: Vec<Box<Node>>,
}

impl Group {
    /// 新規に Group を作成する
    pub fn new() -> Self {
        Group { children: vec![] }
    }
}

impl Shape for Group {
    fn add_child(&mut self, child: Box<Node>) {
        self.children.push(child);
    }

    fn child_at(&self, idx: usize) -> &Box<Node> {
        &self.children[idx]
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

    fn local_normal_at(&self, _p: &Point3D, _: &Intersection) -> Vector3D {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{sphere::Sphere, transform::Transform, FLOAT},
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

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut g1 = Node::new(Box::new(Group::new()));
        g1.set_transform(Transform::rotation_y(
            std::f64::consts::FRAC_PI_2 as FLOAT,
        ));
        let mut g2 = Node::new(Box::new(Group::new()));
        g2.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let mut s = Node::new(Box::new(Sphere::new()));
        s.set_transform(Transform::translation(5.0, 0.0, 0.0));
        let s_ptr = &*s as *const Node;

        g2.add_child(s);
        g1.add_child(g2);

        let p = unsafe {
            s_ptr
                .as_ref()
                .unwrap()
                .world_to_object(&Point3D::new(-2.0, 0.0, -10.0))
        };
        assert_eq!(Point3D::new(0.0, 0.0, -1.0), p);
    }

    #[test]
    fn converting_a_normal_from_world_to_object_space() {
        let mut g1 = Node::new(Box::new(Group::new()));
        g1.set_transform(Transform::rotation_y(
            std::f64::consts::FRAC_PI_2 as FLOAT,
        ));
        let mut g2 = Node::new(Box::new(Group::new()));
        g2.set_transform(Transform::scaling(1.0, 2.0, 3.0));
        let mut s = Node::new(Box::new(Sphere::new()));
        s.set_transform(Transform::translation(5.0, 0.0, 0.0));
        let s_ptr = &*s as *const Node;

        g2.add_child(s);
        g1.add_child(g2);

        let n = unsafe {
            s_ptr.as_ref().unwrap().normal_to_world(&Vector3D::new(
                (3.0 as FLOAT).sqrt() / 3.0,
                (3.0 as FLOAT).sqrt() / 3.0,
                (3.0 as FLOAT).sqrt() / 3.0,
            ))
        };
        assert_eq!(Vector3D::new(0.285714, 0.428571, -0.857143), n);
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut g1 = Node::new(Box::new(Group::new()));
        g1.set_transform(Transform::rotation_y(
            std::f64::consts::FRAC_PI_2 as FLOAT,
        ));
        let mut g2 = Node::new(Box::new(Group::new()));
        g2.set_transform(Transform::scaling(1.0, 2.0, 3.0));
        let mut s = Node::new(Box::new(Sphere::new()));
        s.set_transform(Transform::translation(5.0, 0.0, 0.0));
        let s_ptr = &*s as *const Node;

        g2.add_child(s);
        g1.add_child(g2);

        let n = unsafe {
            s_ptr.as_ref().unwrap().normal_at(
                &Point3D::new(1.7321, 1.1547, -5.5774),
                &Intersection {
                    t: 0.0,
                    object: &g1,
                    u: 0.0,
                    v: 0.0,
                },
            )
        };
        assert_eq!(Vector3D::new(0.2857, 0.428543, -0.85716), n)
    }
}
