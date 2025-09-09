


/*struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a node to the front of the list
    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove the first node with the given value
    fn remove(&mut self, input: i32) {
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

    //search for the data
    fn search(&mut self, input: i32) -> bool{
        let mut current = &mut self.head;
        while let Some(node) = current{
            if node.data == input{
                return true;
            }
            current = &mut node.next;
        }
        return false;
    }
    
    // find the length 
    fn length(&mut self) -> usize{
        let mut current = self.head.take();
        let mut count = 0;

        while let Some(node) = current {
            count +=1;
            current =  node.next;
        }
        return count;
    }

    fn reverse(&mut self){
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current{
            let  next_node = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next_node;
        }
        self.head = prev;
    }

    // Print the list
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push(3);
    list.push(4);
    list.push(1);
    
    list.length();
    println!("Initial list:");
    list.print();

    let value = 2;
    if list.search(value) == true {
        println!("Value {} found in the list.", value);
    }else{
        println!("Value {} not found in the list.", value);
    }

    // Remove a node
    list.remove(4);
    println!("List after removing 4:");
    list.print();

    // Remove a node that doesn't exist
    list.remove(5);
    println!("List after attempting to remove 5:");
    list.print();

    println!("Reversing the list...");
    list.reverse();
    list.print();
}*/
