## Intro
It is used for ``error propagation``(?)  in functions that return a **Result or Option**

When applied to an expression that returns a **Result or Option**
    
- If the value is **ok(or Some)**,it unwraps and gives you the inner value.
- If the value is **Err(or None)**, it returns early from the current function with that error(or None)

Current Example

```rust
pub fn create_new_file(file_name: &str)-> Result<File, std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(b"file created using the std library\n")?;
    Ok(file)
}
```
If you see the return type of `File::create()`
```rust
std::fs
impl File
pub fn create<P: AsRef<Path>>(path: P) -> io::Result<File>
```
the return type is `Result` type and in the documentation statement

>  this function may fail if the full directory path does not exist.

So the error might occur on this statement that's why we added the **try (?)** operator
. Same applies to `file.write_all`

##### Trying to give empty as a file name
```rust
let input_file_path = "";
match file_reader::create_new_file(input_file_path){
     Ok(msg)=>{
            println!("Successfully created a file :{:?}", msg);
        }
        Err(e) =>{
            println!("Failed to create a file error: {:?}", e);
        }
    }
```
Based on our understanding from the documentation it will validate the if the directory path is not given

**Error**:
> Failed to create a file error: Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }

This code ``let mut file = File::create(file_name)?;`` is directly raising the error

**In python we have a thing like we can catch the exception and raise it if we wanted**

**This does not throw an exception.**

Instead:
 - File::open returns a Result<File, Error>
 - ? checks whether it's Ok or Err
 - If it's Err → returns that Err from the current function

```rust
let f = File::open("file.txt")?;
```

python
```python
try:
    f = open("file.txt")
except Exception as e:
    raise e   # bubble upward

```



----


#### Trying with Option return type
What I thought is if we could use the Result as a return type and on the file split add the try operator ?

```rust
use std;

pub fn find_extension_using_split(file_path: &str) -> Result<String, std::io::Error> {
    println!("Checking file pattern using the string split method");
    let extension = file_path.rsplit('.').next()?; // return type is option
    Ok(extension.to_string()) // return type is Result
}
```
But it gave the error like ``you are returning the Option in the code`` but the function return type is Result

##### Error:
>The `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result` [E0277]

Then I modified the `Ok()` statement to match the next return type

```rust
use std;

pub fn find_extension_using_split(file_path: &str) -> Result<String, std::io::Error> { // this one is result type
    println!("Checking file pattern using the string split method");
    let extension = file_path.rsplit('.').next()?; // return type is option
    Some(extension.to_string()) // return type is Option
}
```
##### Error
> Type mismatch [E0308]
>Expected:
>Result<String, std::io::error::Error>
>Found:
>Option<String>
> 

Modified the function return type


##### Correct way of writing
```rust
pub fn find_extension_using_split(file_path: &str) -> Option<String>{
    println!("Checking file pattern using the string split method");
    let extension = file_path.rsplit('.').next()?;
    Some(extension.to_string())
}
```
