## impl
Methods in rust are defined in <b>impl</b> block, which is must be associated with either:

- a struct
- an enum
- or a trait

impl block tells the compiler

> I am about to implement methods or associated functions for this specific type.

so:

if you create a `struct as Rectangle`, you must write `impl Rectangle`

Otherwise rust don't know which type those methods belong to. 

```rust
impl Rectangle{

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
For the area function `rect1` will go as an `argument`

### Function calling look like

area funcion is defined as below
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```
calling

```rust
let rect1 =  Rectangle{ width:50, height:50};

// to access area function

area(&rect1);

// we passed the rect1 as an argument
```

### Turning this into method calling

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

calling

```rust
println!("The area is {} square pixels.", rect1.area());
```

#### This means:

<strong> You start with an instance (like rect1).

Then you use a dot (.).

Then the method name (area).

Then parentheses — () — possibly with arguments inside. </strong>

So:

`rect1.area()`
is method syntax — short for
`Rectangle::area(&rect1)`

### Associate Functions
These are used to create a new data
- Associate functions has `no self`
- Called on the type itself
- Similar to constructor in other languages

##### Define
```rust
#[derive(Debug)]
struct Person {
    age: u32,
    gender: char,
    address: String,
}

impl Person {
    // Associated function
    fn new(age: u32, gender: char, address: &str) -> Person {
        Person {
            age,
            gender,
            address: address.to_string(),
        }
    }

    // here return type Person explicitly given instead of that we can use the Self as well

    // another way of writing it in generic way

    fn new(age: u32, gender: char, address: &str) -> Self {
        Self {
            age,
            gender,
            address: address.to_string(),
        }
    }

    /*
    you can create function with any name it is not mandatory to use the new as a function name
    */
```
Here new is a associate function

##### how do we call associate function

```rust
let person1 = Person::new(49, 'M', "India");
```

### self vs Self

In rust `self` refers to the current objects instance in this case `person1`

`Self` refers to the `class name(Person/Rectangle...etc)` the type itself
