pub struct Node {
    pub data: i32,
    pub prev: Option<Box<Node>>,
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

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} <-> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}
