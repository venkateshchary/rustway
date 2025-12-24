
## println!
It is a **macro**

## Debug
It is a **trait** enables us to print our data type/ variable in a way it is useful for the developers
so we can see its value while we're debuggind the code

## Struct
Unlike other data types(tuple, array , char and string) struct doesn't have debug trait enabled by default.

But we have to explicitly opt in to make that functionality available for our struct

To do debug we need to add the outer attribute Debug **#[derive(Debug)]** on top of struct



Rust intentionally does not support named arguments in functions.
```rust
pub fn new(curry: Curry, cust_name: String) -> Self
```
new(...) is a function, not a struct.

👉 Functions do not accept named arguments in Rust.

```rust
let customer1 = crate::menu::Lunch::new(
    customer_name: String::from("Venkatesh"),
    curry: crate::menu::Curry::NonVeg(Chicken),
);
```
**Syntax (field_name: value) is allowed ONLY when you create a struct directly:**