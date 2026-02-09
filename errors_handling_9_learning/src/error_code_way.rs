// use std::fs::File;
// use std::io::ErrorCode;

/*
 * Below code won't compile because error code is different to OS(windows/linux/mac)
 * So better don't use it in the below
 *
 */
// pub fn file_exist_check_with_error_code(file_path: &str) -> Result<File, ErrorCode> {
//     match File::open(file_path) {
//         Ok(file) => {
//             println!("File found");
//             Ok(file)
//         }
//         Err(err) => {
//             if err.code() == ErrorCode::NotFound {
//                 println!("File not found");
//                 Err(ErrorCode::NotFound)
//             } else {
//                 println!("Error opening file: {:?}", err);
//                 Err(err.code())
//             }
//         }
//     }
// }
