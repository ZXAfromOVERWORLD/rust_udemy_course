//use std::collections::LinkedList;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T>{
    pub fn new()->Self{
        LinkedList(None)
    }

    pub fn push_to_front(&mut self, data: T){
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_to_back(&mut self, data: T){
        match self.0{
            Some((_,ref mut child)) => child.push_to_back(data),
            None => self.push_to_front(data),
        }
    }
}

fn main(){
    println!("Hello World");
    //let list = LinkedList::from([2,4,6,23,5]);
    let mut l: LinkedList<u32> = LinkedList::new();
    //println!("{:?}",list);
    println!("{:?}",l);
    l.push_to_front(23);
    l.push_to_back(2);
    l.push_to_front(65);
    println!("{:?}",l);
}