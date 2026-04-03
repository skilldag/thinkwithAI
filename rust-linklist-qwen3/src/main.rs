use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    fn delete(&mut self, value: i32) {
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.value == value {
                drop(node_ref);
                if let Some(ref prev_node) = prev {
                    prev_node.borrow_mut().next = node.borrow().next.clone();
                } else {
                    self.head = node.borrow().next.clone();
                }
                return;
            }
            drop(node_ref);
            prev = Some(node.clone());
            current = node.borrow().next.clone();
        }
    }

    fn print(&self) {
        let mut current = self.head.clone();

        while let Some(node) = current {
            print!("{} ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut list = LinkedList::new();

    for i in 1..=5 {
        list.insert(i);
    }

    list.delete(3);
    list.print();
}
