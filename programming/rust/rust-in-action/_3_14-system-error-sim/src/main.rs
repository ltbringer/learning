use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: vec![],
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        return f;
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut data_ = self.data.clone();
        let read_length = data_.len();
        save_to.reserve(read_length);
        save_to.append(&mut data_);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(2) {
        let err = String::from("Permission denied");
        return Err(err);
    }
    return Ok(f);
}

fn close(f: File) -> Result<File, String> {
    if one_in(100) {
        let err = String::from("SIGINT");
        return Err(err);
    }
    return Ok(f);
}

fn main() {
    let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("x.txt", &f_data);
    let mut buffer: Vec<u8> = vec![];

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{} is {} bytes long.", &f.name, f_length);
    println!("{}", text);
}
