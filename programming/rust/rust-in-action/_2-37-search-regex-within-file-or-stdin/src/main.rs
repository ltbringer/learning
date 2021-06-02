use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml;

#[derive(Deserialize)]
struct Package<'a> {
    authors: Vec<&'a str>,
    edition: String,
    name: String,
    version: String,
}

struct Dependencies<'a> {
    clap: &'a str,
    regex: &'a str,
    toml: &'a str,
}

struct CargoToml<'a> {
    package: Package<'a>,
    dependencies: Dependencies<'a>,
}

fn read_toml() -> String {
    let cargo_toml_file = File::open("./Cargo.toml").unwrap();
    let cargo_toml_reader = BufReader::new(cargo_toml_file);
    let mut content = "".to_owned();

    for line_ in cargo_toml_reader.lines() {
        let line = line_.unwrap();
        content += &line;
        content += "\n";
    }

    let parsed_cargo_toml: CargoToml = toml::from_str(&content);

    return match parsed_cargo_toml["package"].as_str() {
        Some(thing) => thing.to_owned(),
        None => "Nothing found".to_owned(),
    };
}

fn main() {
    println!("{}", read_toml())
}
