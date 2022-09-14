#[derive(Debug, Default)]
pub struct Node<T> {
    value: &T,
    parent: &Node<T>,
    left_child: &Node<T>,
    right_child: &Node<T>,
}

impl Node<T> {
    pub fn new(value: T, parent: &Node<T>, left_child: &Node<T>, right_child: &Node<T>) -> Self {
        Node {
            value,
            parent,
            left_child,
            right_child,
        }
    }
}
