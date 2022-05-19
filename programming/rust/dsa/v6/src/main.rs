#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>
}

impl <T:std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 4,
            next: None
        }))
    };

    println!("{:?}", ll);

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    println!("{:?}", ll);

    let mut vector: Vec<String> = Vec::with_capacity(100);
    vector.push("hello".to_string());
    vector.push("world".to_string());
    for i in 0..105 {
        vector.push(format!("{}", i));
    }
    println!("v.len = {}, cap = {}", vector.len(), vector.capacity());
}
