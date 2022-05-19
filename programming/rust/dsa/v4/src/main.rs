#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    // works
    let mut x = 34;
    let y = x;
    x += 5;
    println!("y = {}, x = {}", y, x);

    let p1 = Person {
        name: String::from("John"),
        age: 23,
    };
    
    // doesn't work
    // let p2 = p1;
    // println!("p1 = {:?}, p2 = {:?}", p1, p2);
    // While i32 implements Copy trait,
    // which means that it can be copied into a variable of type i32.

    // By derive(Clone) we allow the object to be cloned.
    // However, we can't still copy objects that don't implement the Copy trait.
    // In this case, String doesn't allow copying. So we can clone at best.
    let mut p2 = p1.clone();
    p2.name.push_str(" Smith");
    println!("p1 = {:?}\np2 = {:?}", p1, p2);

    // Here, i32 implements Copy so we can Clone Point and also Copy 
    // its members. 
    let pt1 = Point::new(3, 4);
    let mut pt2 = pt1;
    pt2.x = 5;
    println!("pt1 = {:?}, pt2 = {:?}", pt1, pt2);
}
