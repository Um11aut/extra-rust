pub struct Node<T: std::fmt::Display + std::cmp::PartialEq> {
    element: T,

    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display + std::cmp::PartialEq> Node<T> {
    pub fn new(element: T) -> Self {
        return Node {
            element,
            left: None,
            right: None,
        };
    }

    pub fn set_right(&mut self, value: Node<T>) {
        self.right = Some(Box::new(value));
    }

    pub fn set_left(&mut self, value: Node<T>) {
        self.left = Some(Box::new(value));
    }

    pub fn are_equal(first: &Self, second: &Self) -> bool {
        if first.element != second.element {
            return false;
        }

        let left_equal = match (&first.left, &second.left) {
            (Some(ref f_left), Some(ref s_left)) => Node::are_equal(f_left, s_left),
            (None, None) => true,
            _ => false,
        };

        let right_equal = match (&first.right, &second.right) {
            (Some(ref f_right), Some(ref s_right)) => Node::are_equal(f_right, s_right),
            (None, None) => true,
            _ => false,
        };

        left_equal && right_equal
    }

    pub fn print(node: Box<Node<T>>, depth: usize) {
        for _ in 0..depth {
            print!("    ");
        }

        println!("{}", node.element);

        if node.right.is_some() {
            let temp_depth = depth + 1;
            Node::print(node.right.unwrap(), temp_depth);
        }
        if node.left.is_some() {
            let temp_depth = depth + 1;
            Node::print(node.left.unwrap(), temp_depth);
        }
        return;
    }
}
