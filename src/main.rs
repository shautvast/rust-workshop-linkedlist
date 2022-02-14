fn main() {
    let mut list = List::new();
    list.push(1);
    println!("push(1) : {:?}", list);
    list.push(2);
    println!("push(2) : {:?}", list);
    println!("pop() -> {} : {:?}",list.pop().unwrap(),list);
    println!("pop() -> {} : {:?}",list.pop().unwrap(),list);
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();
        list.push(42);
        println!("{:?}", list);
    }
}
