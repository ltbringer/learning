#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }

    pub fn age_up(&mut self, n: u8) {
        self.age += n;
    }

    pub fn dropme(self) {

    }
}

pub fn get_age(s: &Person) -> &u8 {
    &s.age
}

fn main() {
    let mut p = Person::new(String::from("John"), 23);
    let s1 = p.greet();
    println!("{}", s1);

    let s2 = p.greet();
    println!("also {}", s2);

    let a = get_age(&p);
    println!("Age for {:?} is {}", p, a);
    p.age_up(2);
}
