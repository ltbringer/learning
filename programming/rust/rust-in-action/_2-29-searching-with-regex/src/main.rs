use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("Search for patterns in body")
        .arg(Arg::with_name("file")
            .help("The file path in which we find keywords.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("keyword")
            .help("This value will be found in a body of text.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let file_path = args.value_of("file").unwrap();
    let file_handle = File::open(file_path).unwrap();
    let reader = BufReader::new(file_handle);
    
    let keyword = args.value_of("keyword").unwrap();
    let re = Regex::new(keyword).unwrap();

    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("[👁️‍🗨️@({}).line:{}] :: {:?}", file_path, i , line.trim()),
            None => ()
        }
    }
}
