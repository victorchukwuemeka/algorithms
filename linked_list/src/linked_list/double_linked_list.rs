use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

pub struct Node {
    pub data: i32,
    pub prev: Weak<>,
    pub next: Option<Box<Node>>,
}

pub struct DoubleLinkedList {
    pub head: Option<Box<Node>>,
    pub tail: Option<Box<Node>>,
}

impl DoubleLinkedList {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: i32) {
        let mut new_node = Box::new(Node {
            data,
            prev: None,
            next: self.head.take(),
        });

        if let Some(ref mut old_head) = new_node.next {
            old_head.prev = Some(Box::new(Node {
                data: new_node.data,
                prev: None,
                next: None,
            }));
        }

        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, data: i32){
        let mut new_node = Box::new(Node {
            data,
            prev: None,
            next: self.tail.take(),
        });

        if let Some(ref mut old_tail) =  new_node.next{
            old_tail.prev = Some(Box::new(Node {
                data: new_node.data,
                prev:None,
                next: None,
            }));
        }
        self.tail = Some(new_node);
    }

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} <-> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

pub struct Node {
    pub data: i32,
    pub prev: Weak<RefCell<Node>>,
    pub next: Link,
}

pub struct DoubleLinkedList {
    pub head: Link,
    pub tail: Link,
}

impl DoubleLinkedList {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: Weak::new(),
            next: self.head.clone(),
        }));

        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Rc::downgrade(&new_node);
            new_node.borrow_mut().next = Some(head.clone());
            self.head = Some(new_node);
        } else {
            // list was empty
            self.tail = Some(new_node.clone());
            self.head = Some(new_node);
        }
    }

    pub fn push_back(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: Weak::new(),
            next: None,
        }));

        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Rc::downgrade(&tail);
            self.tail = Some(new_node);
        } else {
            // list was empty
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    pub fn print_forward(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} <-> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("None");
    }

    pub fn print_backward(&self) {
        let mut current = self.tail.clone();
        while let Some(node) = current {
            print!("{} <-> ", node.borrow().data);
            current = node.borrow().prev.upgrade();
        }
        println!("None");
    }
}
