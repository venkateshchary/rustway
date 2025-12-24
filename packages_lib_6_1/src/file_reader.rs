/*
    I am using a standard library to create a file
 */
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum FileTypes{
    Txt,
    Json,
    Csv

}
pub fn check_file_pattern(file_path: &str) -> Option<FileTypes>{
    println!("Checking file pattern");
    if file_path.ends_with("txt"){
        Some(FileTypes::Txt)
    }else if file_path.ends_with("json"){
        Some(FileTypes::Json)
    }else if file_path.ends_with("csv") {
        Some(FileTypes::Csv)
    }else {
        None
    }
}

pub fn find_extension_using_split(file_path: &str) -> Option<String>{
    println!("Checking file pattern using the string split method");
    let extension = file_path.rsplit('.').next()?;
    Some(extension.to_string())
}


pub fn is_file_exist(file_path: &str) -> bool{
    if file_path.is_empty(){
        false
    }else {
        true
    }
}
/*
? must return Err(e) to the caller
 */

/*
     If you don't want to use the ? you can explicitly write match flow with ok and error
     scenario 1:
        I am trying to pass empty file name as param it is raising an error
        Failed to create a file error: Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }

 */
pub fn create_new_file_1(file_name: &str)-> Result<File, std::io::Error>{
    let mut file = File::create(file_name);
    // file.write_all(b"file created using the std library\n")?;
    match file {
        Ok(file) => {Ok(file)},
        Err(e) => {
            println!("Failed to create file: {:?}", e);
            Err(e)
        }
    }
    // file.write_all(b"file created using the std library\n")?;
    // Ok(file)
}

pub fn create_new_file(file_name: &str)-> Result<File, std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(b"file created using the std library\n")?;
    Ok(file)
}