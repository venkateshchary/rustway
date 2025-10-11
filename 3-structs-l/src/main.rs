#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    /*
        without structs using the variables finding the area of rectangle
    */
    let width = 50;
    let height = 50;
    println!("width: {} height: {}", width, height);
    println!("area of rectangle: {}", area(width, height));
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    /*
    problem:  the function we wrote has two parameters,
     and it’s not clear anywhere in our program that the parameters are related.
     It would be more readable and more manageable to group width and height together
     */

    // using the tuples

    let dimension: (u32, u32) = (50, 50);
    println!("area using the tupe :{}", area_t(dimension));

    fn area_t(dimension: (u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }

    /*
    problem:  this version is less clear: tuples don’t name their elements
     */

    // using the structs
    let reactangle_1 = Rectangle {
        width: 50,
        height: 50,
    };
    println!("area of a rectangle: {}", area_r(&reactangle_1));

    fn area_r(react1: &Rectangle) -> u32 {
        react1.width * react1.height
    }

    /*
        Are you able to print the Rectangle struct using the println macro
        error: it will raise an below error
        the trait `std::fmt::Display` is not implemented for `Rectangle`
    */

    // println!("rect1 is {reactangle_1}");

    // try println! with existing primitive types if it is working or not

    let a: char = 'c';
    let name: String = String::from("venkatesh");
    let ar:[u32;2] = [1,2];
    let t:(u32, char) = (45, 'c');
    println!("charcter display check: {a}");
    println!("tuple display check: {t:?}"); // the trait `std::fmt::Display` is not implemented for `(u32, char)`
    println!("array display check: {ar:?}"); // // the trait `std::fmt::Display` is not implemented for 
    println!("string display check: {name:?}");
    /*
        :? inside the curly brackets tells println! we want to use an output format called Debug
    */

    // after adding the debug for the struct lets print it

    println!("rectangle struct to view: {reactangle_1:?}"); // output: rectangle struct to view: Rectangle { width: 50, height: 50 }
    /*
        lets learn what # will do in println
     */
    println!("rectagle struct in prettu printed way: {reactangle_1:#?}"); 
    /*
        output:
        rectagle struct in prettu printed way: Rectangle {
                                                    width: 50,
                                                    height: 50,
                                                }
     */




}
