use crate::{shape::Shape, transform::Transform};
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
}

pub fn add_child(parent: &mut Box<Node>, mut child: Box<Node>) {
    child.parent = NonNull::new(&mut **parent);
    //    parent.shape.add_child(child);
}
