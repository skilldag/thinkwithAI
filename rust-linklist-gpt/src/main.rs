fn main() {
    let mut list = rust_linked_list::LinkedList::new();

    for i in 1..=5 {
        list.insert(i);
    }

    list.delete(3);
    list.print(); // Should print 1 2 4 5
}
