mod modules;

use modules::list;
use modules::node;

fn main() {
    let mut list = list::List::new(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);

    list.print();

    list.dublicate_values();
    list.set(3, 2);
    list.print();

    let mut root = node::Node::new('q');
    root.set_left(node::Node::new('+'));
    root.set_right(node::Node::new('-'));

    if let Some(ref mut left) = root.left {
        left.set_left(node::Node::new('4'));
        left.set_right(node::Node::new('3'));
    }

    if let Some(ref mut right) = root.right {
        right.set_left(node::Node::new('5'));
        right.set_right(node::Node::new('1'));
    }

    let mut root1 = node::Node::new('q');
    root1.set_left(node::Node::new('+'));
    root1.set_right(node::Node::new('-'));

    if let Some(ref mut left) = root1.left {
        left.set_left(node::Node::new('4'));
        left.set_right(node::Node::new('3'));
    }

    if let Some(ref mut right) = root1.right {
        right.set_left(node::Node::new('5'));
        right.set_right(node::Node::new('1'));
    }
    let test1 = node::Node::are_equal(&root, &root1);

    println!("{test1}");

    node::Node::print(Box::new(root), 0);
}
