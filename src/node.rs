use crate::{
    intersection::Intersection, material::Material, point3d::Point3D, ray::Ray,
    shape::Shape, transform::Transform, vector3d::Vector3D,
};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node {
    parent: Option<NonNull<Node>>,
    transform: Transform,
    shape: Box<dyn Shape>,
}

impl Node {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Node {
            parent: None,
            transform: Transform::identity(),
            shape,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn material(&self) -> &Material {
        self.shape.material()
    }

    pub fn material_mut(&mut self) -> &mut Material {
        self.shape.material_mut()
    }

    /// ray と self の交点を求める。全ての交点を Vec に入れて返す。
    /// 交点がない場合には空の Vec を返す。
    ///
    /// # Argumets
    /// * `ray` - 交点の計算対象となる Ray
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let local_ray = self.transform.inv() * r;
        self.shape.local_intersect(&local_ray, self)
    }

    /// self 上の点 p における法線ベクトルを取得する。
    ///
    /// # Argumets
    /// * `p` - self 上の点
    pub fn normal_at(&self, p: &Point3D) -> Vector3D {
        let p_in_local = self.transform.inv() * p;
        let n = self.shape.local_normal_at(&p_in_local);

        self.transform.apply_to_normal(&n)
    }
}

pub fn add_child(parent: &mut Box<Node>, mut child: Box<Node>) {
    child.parent = NonNull::new(&mut **parent);
    parent.shape.add_child(child);
}

#[cfg(test)]
mod tests {
    use super::{super::group::Group, *};

    #[test]
    fn creating_a_new_node() {
        let g = Group::new();
        let n = Node::new(Box::new(g));

        assert_eq!(Transform::identity(), n.transform);
    }

    #[test]
    fn the_default_transformation() {
        let node = Node::new(Box::new(Group::new()));

        assert_eq!(*node.transform(), Transform::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut node = Node::new(Box::new(Group::new()));
        node.set_transform(Transform::translation(2.0, 3.0, 4.0));

        assert_eq!(*node.transform(), Transform::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn a_node_has_a_parent_attribute() {
        let g = Node::new(Box::new(Group::new()));
        assert_eq!(None, g.parent);
    }
}
