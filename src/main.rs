fn main() {
    
}

pub struct List {
    head: Link
}

enum Link{
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }
}

