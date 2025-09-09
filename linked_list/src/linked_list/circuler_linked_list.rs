pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

pub struct CircularLinkedList {
    pub head: Option<Box<Node>>,
}

impl CircularLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });

        if self.head.is_none() {
            // first node points to itself
            let mut head_node = new_node;
            head_node.next = Some(Box::new(Node { data, next: None }));
            self.head = Some(head_node);
        }
    }
}
