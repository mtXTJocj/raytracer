use crate::{
    intersection::Intersection, material::Material, point3d::Point3D, ray::Ray,
    shape::Shape, transform::Transform, vector3d::Vector3D,
};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node {
    /// 親 Node
    parent: Option<NonNull<Node>>,
    /// 親 Node の座標系への変換
    transform: Transform,
    /// 本体
    shape: Box<dyn Shape>,
}

impl Node {
    /// 新規に Node を作成する
    ///
    /// # Argumets
    /// * `shape` - この Node 固有の性質となる Shape
    pub fn new(shape: Box<dyn Shape>) -> Box<Self> {
        Box::new(Node {
            parent: None,
            transform: Transform::identity(),
            shape,
        })
    }

    /// 子 Node を追加する
    ///
    /// # Argumets
    /// * `child` - 追加する Node
    pub fn add_child(&mut self, mut child: Box<Node>) {
        child.parent = NonNull::new(&mut *self);
        self.shape.add_child(child);
    }

    pub fn child_at(&self, idx: usize) -> &Box<Node> {
        self.shape.child_at(idx)
    }

    /// 親 Node の座標系への変換を取得する
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// 親 Node の座標系への変換を設定する
    ///
    /// # Argumets
    /// * `transform` - 設定する Transform
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    /// World 座表系の点 p から self の local 座標系の点を求める
    ///
    /// # Argumets
    /// * `p` - World 座表系の点 p
    pub(crate) fn world_to_object(&self, p: &Point3D) -> Point3D {
        match self.parent {
            None => self.transform().inv() * p,
            Some(n) => unsafe {
                self.transform().inv() * &n.as_ref().world_to_object(p)
            },
        }
    }

    /// local 座表系の法線ベクトル n から World 座標系の法線ベクトルを求める
    ///
    /// # Argumets
    /// * `p` - World 座表系の点 p
    pub(crate) fn normal_to_world(&self, n: &Vector3D) -> Vector3D {
        match self.parent {
            None => self.transform.apply_to_normal(n),
            Some(node) => unsafe {
                node.as_ref()
                    .normal_to_world(&(self.transform().apply_to_normal(n)))
            },
        }
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
        let local_point = self.world_to_object(p);
        let local_normal = self.shape.local_normal_at(&local_point);

        self.normal_to_world(&local_normal)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::group::Group, *};

    impl Node {
        pub(crate) fn shape(&self) -> &Box<dyn Shape> {
            &self.shape
        }
    }

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
