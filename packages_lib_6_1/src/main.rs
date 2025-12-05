mod file_reader;

fn main() {
    println!("Hello, world!");
    let input_file_path = "employee.txt";
    // file_reader::check_file_pattern();
    match file_reader::check_file_pattern(input_file_path) {
        Some(file_type) => {
            println!("file type is : {:?}", file_type);
        }
        None =>{
            println!("No file pattern found");
        }
    }
}
