use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Closed => write!(f, "CLOSED"),
            FileState::Open => write!(f, "OPEN"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: vec![],
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open before it can be read."));
        }
        let mut data_ = self.data.clone();
        let data_size = data_.len();
        save_to.reserve(data_size);
        save_to.append(&mut data_);
        Ok(data_size)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f = File::new("x.txt");
    let mut buffer: Vec<u8> = vec![];
    if f.read(&mut buffer).is_err() {
        println!("Exception was raised!");
    }

    f = open(f).unwrap();
    let f_size = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{} has size {}", f.name, f_size);
    println!("{}", text);
}
