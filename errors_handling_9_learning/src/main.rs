use std::fs::File;
use std::io::ErrorKind;

// mod error_code_way;
mod error_way;
mod file_reader;
mod panic_way;
mod json_file_reader;
mod list_users_reader;

// use error_code_way::file_exist_check_with_error_code;
use error_way::{open_file_with_if_case, open_file_with_match_case};

use file_reader::{read_file,
     read_file_with_question,
     file_reader_in_shorter_way
    };

use panic_way::{panic_with_custom_msg};
use json_file_reader::{json_reader};
use list_users_reader::{reader_list_file};


fn main() {
    println!("Hello, world!");
    let file = File::open("data/users1.json");
    let file_data: Result<File, std::io::Error> = match file {
        Ok(file) => {
            println!("File opened successfully");
            Ok(file)
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found");
                Err(error)
            }
            ErrorKind::PermissionDenied => {
                println!("Permission denied");
                Err(error)
            }
            _ => Err(error),
        },
    };

    println!("file data: {:?}", file_data);

    let existing_file_path = "data/users.json";
    let non_existing_file_path = "data/non_existing_file.json";

    let result_existing_path_if_case = open_file_with_if_case(existing_file_path);
    let result_existing_path_match = open_file_with_match_case(existing_file_path);
    let result_non_existing_path_if_case = open_file_with_if_case(non_existing_file_path);
    let result_non_existing_path_match = open_file_with_match_case(non_existing_file_path);

    println!(
        "result_existing_path_if_case: {:?}",
        result_existing_path_if_case
    );
    println!(
        "result_existing_path_match: {:?}",
        result_existing_path_match
    );
    println!(
        "result_non_existing_path_if_case: {:?}",
        result_non_existing_path_if_case
    );
    println!(
        "result_non_existing_path_match: {:?}",
        result_non_existing_path_match
    );

    if let Ok(mut user_data) = result_existing_path_if_case {
        println!("User_Data: {:?}", user_data);
        let content_2 = read_file_with_question(&mut user_data);
        match content_2 {
            Ok(content) => println!("File content: {}", content),
            Err(err) => println!("Error reading file: {}", err),
        }
    } else {
        println!("Error opening file");
    }

    if let Ok(mut user_data) = result_existing_path_match {
        println!("User_Data: {:?}", user_data);
        println!("reading the file data:");
        let content = read_file(&mut user_data);
        match content {
            Ok(content) => println!("File content: {}", content),
            Err(err) => println!("Error reading file: {}", err),
        }
    } else {
        println!("Error opening file");
    }

    if let Ok(user) = result_non_existing_path_if_case {
        println!("User: {:?}", user);
    } else {
        println!("Error opening file");
    }

    if let Ok(user) = result_non_existing_path_match {
        println!("User: {:?}", user);
    } else {
        println!("Error opening file");
    }

    let shorter_way_res: Result<String, std::io::Error> = file_reader_in_shorter_way(&existing_file_path);
    println!("shorter way reading data: {:?}", shorter_way_res);
    match file_reader_in_shorter_way(&existing_file_path){
        Ok(file_data) => {
            println!("data: {:?}",file_data);
        }
        Err(error) => {
            println!("unable to read the file data: {:?}", error);
        }
    }
    // if let Ok(file) = file_exist_check_with_error_code(&existing_file_path) {
    //     println!("File found");
    // } else {
    //     println!("File not found");
    // }
    // let result_unwrap = using_unwrap(&existing_file_path);
    // println!("result_unwrap: {:?}", result_unwrap);

    panic_with_custom_msg(&existing_file_path);

    json_reader(&existing_file_path);

    reader_list_file();


}
/*
enum Resutl<T,E> {
   Ok(T),
   Err(E),
   }
*/
