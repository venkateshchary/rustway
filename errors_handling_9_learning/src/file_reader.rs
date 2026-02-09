use std::fs::{self, File};
use std::io::Read;

pub fn read_file(file_obj: &mut File) -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    match file_obj.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(err),
    }
}
/*
 * Either of the way we can use it match or ? operator
 */
pub fn read_file_with_question(file: &mut File) -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn file_reader_in_shorter_way(file_path: &str) -> Result<String, std::io::Error>{
    fs::read_to_string(file_path)
}
