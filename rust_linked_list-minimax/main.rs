use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn delete(&mut self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut prev_next = &mut self.head;
        loop {
            match prev_next {
                Some(node) if node.value == *value => {
                    *prev_next = node.next.take();
                    return true;
                }
                Some(node) => {
                    prev_next = &mut node.next;
                }
                None => return false,
            }
        }
    }

    pub fn print(&self)
    where
        T: Display,
    {
        let mut current = &self.head;
        print!("LinkedList: ");
        while let Some(ref node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.insert(5);
    list.insert(4);
    list.insert(3);
    list.insert(2);
    list.insert(1);

    println!("After inserting 1-5:");
    list.print();

    list.delete(&3);

    println!("After deleting 3:");
    list.print();
}
