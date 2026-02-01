use std::fs::File;
use std::io::ErrorKind;

mod error_way;

use error_way::{open_file_with_if_case, open_file_with_match_case};

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

    if let Ok(user_data) = result_existing_path_if_case {
        println!("User_Data: {:?}", user_data);
    } else {
        println!("Error opening file");
    }

    if let Ok(user) = result_existing_path_match {
        println!("User: {:?}", user);
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
}
/*
enum Resutl<T,E> {
   Ok(T),
   Err(E),
   }
*/
