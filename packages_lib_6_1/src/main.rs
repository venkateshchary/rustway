mod file_reader;

fn main() {
    println!("Hello, world!");
    let input_file_path = "";
    match file_reader::create_new_file(input_file_path){
        Ok(msg)=>{
            println!("Successfully created a file :{:?}", msg);
        }
        Err(e) =>{
            println!("Failed to create a file error: {:?}", e);
        }
    }

    // file_reader::check_file_pattern();
    match file_reader::check_file_pattern(input_file_path) {
        Some(file_type) => {
            println!("file type is : {:?}", file_type);
        }
        None =>{
            println!("No file pattern found");
        }
    }

    println!("Verify the file extension using the split method");

    let file_extension = file_reader::find_extension_using_split(input_file_path);
    println!("File extension: {:?}", file_extension); // File extension: Some("")
    // here we can use the match statement instead of direct println!

    match file_extension {
        Some(extension) => {
            if extension.is_empty(){
                println!("No file extension found"); // println!("File extension: {:?}", extension);
            }
            else {
                println!("File extension: {:?}", extension);
            }
        }
        None =>{
            println!("No file extension found");
        }
    }

    if file_reader::is_file_exist(input_file_path) {
        println!("File already exists");
    }
}
