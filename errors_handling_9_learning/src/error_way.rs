use std::fs::File;
use std::io::ErrorKind;

pub fn open_file_with_if_case(file_path: &str) -> Result<File, std::io::Error> {
    /*
     * I wanted to open a file,
     * if file is not exists then throw an error
     * if file exists but user doesn't have permission to read it, throw an error
     */
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            println!("File exists and able to open it no errors");
            Ok(file)
        }
        Err(error) => {
            if error.kind() == ErrorKind::PermissionDenied {
                println!("Permission denied");
                Err(error)
            } else if error.kind() == ErrorKind::NotFound {
                println!("File not found");
                Err(error)
            } else {
                println!("Unknown error");
                Err(error)
            }
        }
    }
}

pub fn open_file_with_match_case(file_path: &str) -> Result<File, std::io::Error> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            println!("File exists and able to open it no errors");
            Ok(file)
        }
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                println!("Permission denied");
                Err(error)
            }
            ErrorKind::NotFound => {
                println!("File not found");
                Err(error)
            }
            _ => {
                println!("Unknown error");
                Err(error)
            }
        },
    }
}
