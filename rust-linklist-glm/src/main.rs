use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

pub struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    pub fn delete(&mut self, value: i32) -> bool {
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;

        loop {
            match current {
                None => return false,
                Some(node) => {
                    if node.borrow().value == value {
                        if let Some(mut prev_node) = prev {
                            prev_node.borrow_mut().next = node.borrow().next.clone();
                        } else {
                            self.head = node.borrow().next.clone();
                        }
                        return true;
                    }
                    prev = Some(node.clone());
                    current = node.borrow().next.clone();
                }
            }
        }
    }

    pub fn print(&self) {
        print!("Printing from head to tail: [");
        let mut current = self.head.clone();
        let mut first = true;
        while let Some(node) = current {
            if !first {
                print!(" -> ");
            }
            print!("{}", node.borrow().value);
            first = false;
            current = node.borrow().next.clone();
        }
        println!("]");
    }

    pub fn reversed_print(&self) {
        print!("Printing from tail to head: [");
        let mut values = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().next.clone();
        }
        for (i, val) in values.iter().enumerate() {
            if i > 0 {
                print!(" -> ");
            }
            print!("{}", val);
        }
        println!("]");
    }

    pub fn search(&self, value: i32) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    pub fn size(&self) -> usize {
        self.head.as_ref().map_or(0, |head| {
            let mut current = Some(head);
            let mut count = 0;
            while let Some(node) = current {
                count += 1;
                current = node.borrow().next.clone().as_ref().map(Rc::clone);
            }
            count
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        self.head = None;
    }
}

fn main() {
    println!("=== Rust 单向链表实现 ===\n");

    let mut list = LinkedList::new();

    println!("1. 初始状态:");
    println!("- 空列表: {}", list.is_empty());
    println!("- 大小: {}", list.size());
    println!("- 倒序打印:");
    list.reversed_print();

    println!("\n2. 插入元素 (1, 2, 3, 4, 5):");
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);
    list.insert(5);

    println!("- 正序打印:",);
    list.print();
    println!("- 倒序打印:");
    list.reversed_print();
    println!("- 大小: {}", list.size());

    println!("\n3. 搜索元素:");
    println!(
        "- 搜索 3: {}",
        if list.search(3) {
            "找到"
        } else {
            "未找到"
        }
    );
    println!(
        "- 搜索 6: {}",
        if list.search(6) {
            "找到"
        } else {
            "未找到"
        }
    );

    println!("\n4. 插入元素 0:");
    list.insert(0);

    println!("- 正序打印:");
    list.print();
    println!("- 倒序打印:");
    list.reversed_print();
    println!("- 大小: {}", list.size());

    println!("\n5. 删除元素:");
    println!(
        "- 删除 3: {}",
        if list.delete(3) {
            "成功"
        } else {
            "未找到"
        }
    );
    println!(
        "- 删除 1: {}",
        if list.delete(1) {
            "成功"
        } else {
            "未找到"
        }
    );
    println!(
        "- 删除 0: {}",
        if list.delete(0) {
            "成功"
        } else {
            "未找到"
        }
    );
    println!(
        "- 删除 10: {}",
        if list.delete(10) {
            "成功"
        } else {
            "未找到"
        }
    );
    println!(
        "- 删除 0: {}",
        if list.delete(0) {
            "成功"
        } else {
            "未找到"
        }
    );

    println!("\n6. 最终状态:");
    println!("- 正序打印:");
    list.print();
    println!("- 倒序打印:");
    list.reversed_print();
    println!("- 空列表: {}", list.is_empty());
    println!("- 大小: {}", list.size());

    println!("\n7. 清空列表:");
    list.clear();
    println!("- 空列表: {}", list.is_empty());
    println!("- 大小: {}", list.size());

    println!("\n=== 实现完成 ===");
}
