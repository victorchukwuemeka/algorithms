pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

pub struct SingleLinkedList {
    pub head: Option<Box<Node>>,
}

impl SingleLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }


    pub fn remove(&mut self, input: i32){
        let mut current = &mut self.head;

        while current.is_some() {
            if let Some(node) = current.as_mut(){
                if node.data == input{
                    *current = node.next.take();
                    return;
                }
            }
            current = &mut current.as_mut().unwrap().next;
        }
        println!("Value {} not found in the list.", input);
    }

    pub fn search(&mut self, input: i32)->bool {
        let mut current = &mut self.head;
        
        while let Some(node) = current.as_mut() {
            if node.data == input {
                return true;
            }
            current = &mut node.next;
        }
        return false;
    }



    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

