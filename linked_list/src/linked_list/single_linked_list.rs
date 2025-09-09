use super::node::Node;

struct SingleLinkedList{
    head : Option<Box<Node>>,
}

impl SingleLinkedList{
    fn new ()->Self{
        SingleLinkedList { head: None}
    }

    //add new node  
    //the next node is always pointing to the 
    //old node 
    fn push(&mut self, data:i32){
        let new_node = Box::new(
            Node{
                data: data,
                next: self.head.take(),
            }
        );
        self.head = Some(new_node);
    }
    
}

