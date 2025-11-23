#[allow(unused_variables)]
/*
 IF you declare the variables and not used in where then it will through a warning
 note: `#[warn(unused_variables)]` on by default
 to avoid this warning we need to allow unused_variables
 */

fn main() {
    println!("learn about the data types");
    let a :i32 = 10;
    let b :i32 = 20;
    let c : char = 'c';
    let pi:f32 = 3.14;
    let name: String = String::from("Venkatesh");

    /*
        Checking the copy
        here we are just passing the int type variables
        mean the copy values of a and b will transfer to the add_two_number function
     */
    let add_result = add_two_numbers(a, b);
    println!("add result:{:?}",add_result);

    fn add_two_numbers(v1:i32, v2:i32) -> i32 {
        v1 + v2
    }

    /*
        If we directly pass the variable name to capitalize_name then it is a
        transfer of name completly to capitalize_name function because string datatype
        don't implement the copy trait
        suppose after executing the function if you try to print the name it will raise an error

     */
    let capital_name = capitalize_name(name);
    println!("capital name:{:?}",capital_name);
    fn capitalize_name(name: String)-> String{
        name.to_uppercase()
    }
    /*
    println!("after transferring the name to fn and trying to println name: {:?}",name);
    error: this parameter takes ownership of the value
    */

}
