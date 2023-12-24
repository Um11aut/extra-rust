use std::cell::RefCell;
use std::rc::Rc;

struct Node<T: std::fmt::Display + Clone> {
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T: std::fmt::Display + Clone> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T: std::fmt::Display + Clone> List<T> {
    pub fn new(element: T) -> Self {
        let node = Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }));

        return List {
            head: Some(Rc::clone(&node)),
            tail: Some(node),
        };
    }

    pub fn add(&mut self, element: T) {
        if let Some(tail) = self.tail.take() {
            let new_node = Rc::new(RefCell::new(Node {
                element,
                next: None,
                prev: Some(Rc::clone(&tail)),
            }));

            tail.borrow_mut().next = Some(Rc::clone(&new_node));

            self.tail = Some(new_node);
        }
    }

    pub fn print(&self) {
        let mut current = self.head.clone();

        while let Some(node) = current {
            let prev_value = node
                .borrow()
                .prev
                .as_ref()
                .map_or("".to_string(), |n| n.borrow().element.to_string());
            let current_value = node.borrow().element.to_string();
            let next_value = node
                .borrow()
                .next
                .as_ref()
                .map_or("".to_string(), |n| n.borrow().element.to_string());

            println!(
                "{: >2} {: >2} {: >2}",
                prev_value, current_value, next_value
            );

            current = node.borrow().next.clone();
        }
    }

    pub fn dublicate_values(&mut self) {
        let mut current = self.head.clone();

        while let Some(node) = current.clone() {
            //create node for current to point
            let new_node = Rc::new(RefCell::new(Node {
                element: node.borrow().element.clone(),
                next: node.borrow().next.clone(),
                prev: Some(Rc::clone(&node)),
            }));

            current = node.borrow().next.clone();
            node.borrow_mut().next = Some(Rc::clone(&new_node));
        }
    }
}
