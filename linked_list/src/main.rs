use console::style;
use colored::*;

mod linked_list {
    pub mod single_linked_list;
    pub mod double_linked_list;
    pub mod circuler_linked_list;
}


// Bring the types into scope
use linked_list::single_linked_list::SingleLinkedList;
use linked_list::double_linked_list::DoubleLinkedList;
use linked_list::circuler_linked_list::CircularLinkedList;


fn main() {
    println!("--- Testing Single Linked List ---");
    let mut sll = SingleLinkedList::new();
    sll.push(1);
    sll.push(2);
    sll.push(3);
    sll.push(4);
     println!(
        "{}",
        style("--- Printing the linked list before changes ---")
            .cyan()
            .bold()
            .underlined()
    );
    sll.print();


    sll.search(2);
    sll.remove(3);
    sll.print();

    println!("--- Testing Double Linked List ---");
    let mut dll = DoubleLinkedList::new();
    dll.push_front(10);
    dll.push_front(20);
    dll.print();

    println!("--- Testing Circular Linked List ---");
    let mut cll = CircularLinkedList::new();
    cll.push(100);
    // cll.print();  // implement later
}
