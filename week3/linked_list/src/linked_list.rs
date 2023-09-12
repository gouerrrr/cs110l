use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T:Clone> Clone for LinkedList<T>{
    fn clone(&self)->Self{
        let mut res:LinkedList<T>=LinkedList::new();
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut temp_restore:Vec<T>=Vec::new();
        loop {
            match current {
                Some(node) => {
                    let value=node.value.clone();
                    temp_restore.push(value);
                current=&node.next},
                None => break,
            }
        }
        for i in (0..self.get_size()).rev(){
            res.push_front(temp_restore[i].clone());
        }
        res
    }
    fn clone_from(&mut self, source: &Self) {
        *self=source.clone();
    }
}

impl<T> Drop for LinkedList<T> {
    
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T:std::cmp::Eq> PartialEq for LinkedList<T>{
    fn eq(&self, other: &Self) -> bool {
        let mut current_self=&self.head;
        let mut current_other=&other.head;

        if self.size!=other.size {return false;}
        
            while let (Some(node_self),Some(node_other)) = (current_self,current_other)  {
                if node_self.value!=node_other.value{return false;}
                else{current_self=&node_self.next;current_other=&node_other.next;}
                
            }
            true    
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



