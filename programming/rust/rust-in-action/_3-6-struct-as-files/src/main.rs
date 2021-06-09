#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File {
        name: String::from("./README.md"),
        data: Vec::new(),
    };
    let f_name = &f1.name;
    let f_data = &f1.data.len();
    println!("{} is {} bytes long.", f_name, f_data);
}
