#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn mock_new(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let mock_data = vec![114, 117, 115, 116, 33, 33];
    let mut f = File::mock_new("1.txt", &mock_data);
    let mut buffer: Vec<u8> = vec![];
    open(&mut f);
    let f_length = f.read(&mut buffer);
    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f);
    println!("{} is {} bytes long", &f.name, f_length);
    println!("{}", text);
}
