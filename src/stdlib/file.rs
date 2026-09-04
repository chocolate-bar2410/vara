use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn read_file(path : &str) -> io::Result<String> {

    let mut file = fs::File::open(path).expect(format!("Could not find file at path {}", path).as_str());
    let mut contents : String = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn read_file_with_pathbuf(path : &PathBuf) -> io::Result<String> {

    let mut file = fs::File::open(path).expect(format!("Could not find file at path {}", path.display()).as_str());
    let mut contents : String = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}