use std::fs;
use std::io::{self, Read};

pub fn read_file(path : &str) -> io::Result<String> {

    let mut file = fs::File::open(path).expect(format!("Could not find file at path {}", path).as_str());
    let mut contents : String = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}