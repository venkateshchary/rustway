## RUST DOESN'T HAVE EXCEPTIONS

Rust groups errors into 2 groups
1. **Recoverable**
2. **Unrecoverable**

#### Recoverable
Recoverable errors are errors that can be handled by the program, such as a file not found error.
type
> Result<T, E>

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```
- The T and E are generic types that can be any type.

- T is returned type of value that will be returned in a success case within `Ok`.

- E is returned type of error that will be returned in a failure case within `Err` variant.

**Let us see the below example:**
```rust
let file = File::open("data/users.json");
let file_data: File = match file {
    Ok(file) => {
        println!("File opened successfully");
        file
    },
    Err(error) => {
        panic!("Error opening file: {:?}", error);
    }
};

println!("file data: {:?}", file_data);
}
```

If the file is not found, we can handle it by returning a default value or by prompting the user to enter a valid file path.

You can match the error kinds if the file is not found using the `kind()` method of the `Error` trait.


| ErrorKind           | Meaning                   |
| ------------------- | ------------------------- |
| `NotFound`          | File doesn’t exist        |
| `PermissionDenied`  | No access rights          |
| `ConnectionRefused` | Network connection failed |
| `ConnectionReset`   | Connection dropped        |
| `TimedOut`          | Operation took too long   |
| `InvalidInput`      | Bad input                 |
| `UnexpectedEof`     | File ended unexpectedly   |
| `Other`             | Anything else             |


---

### Unrecoverable
 Unrecoverable errors are errors that cannot be handled by the program, such as a stack overflow.

Unrecoverable errors **always systems symptoms of bugs** such as trying to access a
location beyond the end of an array. and so we wanted to stop
the programme immediately

macro
> panic!

### `?` Operator

? operator in a function that returns Result, Option, or another type that implements FromResidual

 You can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match. The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.