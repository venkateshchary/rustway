## Module split into separate files

Suppose My lib.rs contains below code

```scss
1 top module file_processor
|
|
|- file_reader - sub module
|
|- files - sub module
```
code
>lib.rs 
```rust
mod file_processor{
    
    fn verify_file_exist(){
        
    }
    
    fn verify_file_extension(){
        
    }
    
    mod file_reader{
        
        fn read_file_content(){
            
        }
        
    }
    mod files{
        
        fn create_a_new_file(){
            
        }
        fn rename_file(){
            
        }
    }
}
```
We are going to split this module into multiple files instead of 1 single lib.rs contains the all 
