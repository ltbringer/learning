#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    children: u8,
    fave_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!("name={} age={} children={}", self.name, self.age, self.children)
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

fn main() {
    let p = Person {
        name: "John".to_string(),
        age: 42,
        children: 2,
        fave_color: Color::Green
    };
    match &p.fave_color {
        Color::Red(s) => println!("{}", s),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
    println!("Introducing {:?}", p);
}
