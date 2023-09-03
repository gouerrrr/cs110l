struct Node {
    value: u32,
    next: Option<Box<Node>>,
}
pub struct LinkList {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    pub fn new(input_value: u32, input_next: Option<Box<Node>>) -> Node {
        Node {
            value: (input_value),
            next: (input_next),
        }
    }
}

impl LinkList {
    pub fn new() -> LinkList {
        LinkList {
            head: None,
            size: 0,
        }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn pop(&mut self) -> Option<u32> {
        let temp_node = self.head.take()?; //take fn is used on a mut reference. replace the old reference with None.
        self.head = temp_node.next; //when the reference is not mut, you not need and can't use take().
        self.size -= 1;
        Some(temp_node.value)
    }
    pub fn display(&self) {
        let mut current = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{}{}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        print!("{}", result);
    }
}

fn main() {
    //have the main to test the linklist
    let mut list = LinkList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.display();
    println!();
    println!("size: {}", list.get_size());
    println!("pop: {}", list.pop().unwrap());
    println!("size: {}", list.get_size());
    list.display();
    println!();
}
