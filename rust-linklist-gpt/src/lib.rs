use std::fmt::{self, Debug};

pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(value={})", self.value)
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    len: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn insert(&mut self, val: i32) {
        let mut node = Box::new(Node {
            value: val,
            next: None,
        });
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
    }

    pub fn delete(&mut self, val: i32) -> bool {
        let mut dummy = Box::new(Node {
            value: 0,
            next: self.head.take(),
        });
        let mut prev = &mut dummy;
        while let Some(ref mut curr) = prev.next {
            if curr.value == val {
                prev.next = curr.next.take();
                self.len -= 1;
                self.head = dummy.next;
                return true;
            }
            prev = prev.next.as_mut().unwrap();
        }
        self.head = dummy.next;
        false
    }

    pub fn print(&self) {
        let mut cur = &self.head;
        while let Some(ref node) = cur {
            print!("{} ", node.value);
            cur = &node.next;
        }
        println!();
    }
}
