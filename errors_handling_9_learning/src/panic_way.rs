use std::{fs::File, io::Read, panic};

/*
    We are passing the non existence file to check
    unwrap is able to the panic and stop the program
    
    unwrap will have 2 options Ok(data) or panic

    expect also have 2 options Ok(data) or panic

*/
pub fn using_unwrap(_file_path : &str){
    //let file_data = File::open(file_path).unwrap();
    println!("datap--");
    /*
    Below error it is raised
    thread 'main' panicked at src\panic_way.rs:8:43:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
     */
}

pub fn panic_with_custom_msg(file_path: &str){
    File::open(file_path).expect("File should exists");

    /*
    thread 'main' panicked at src\panic_way.rs:17:27:
File should exists: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\errors_handling_9_learning.exe` (exit code: 101)
    
    
     */
}
