
- ENUM
- Option
- Pattern match expression
- if let construct

### ENUM
> A value of this type can be one of the several possible options


```rust
enum JobStatus{
    Waiting,
    Started,
    InProgress,
    Completed,
    Error,
}
```
<strong>We are defining our own custom data type just like how `i32`, `f64`, `char`, `String` are built-in types </strong>

Here defined a custom type `JobStatus` that can only ever be one of `Started`, `InProgress`, `Completed`, `Error`, `Waiting`

#### How to access it
```rust
let waiting_job_status = JobStatus::Waiting;
// we are created a variable of JobStatus Type
```

##### Note
>Debug trait is not implemented by default for the enum, so we need to derive it to to println!

#### Note
>We can keep any kind of data inside an enum as variant: strings, numeric types, or structs

## Impl blocks
Enum can have impl blocks as well same as struct
We can create multiple constructors instead of one. Struct holds fields, while enum represents variants.

##### Struct:
All instances share the same fields, but fields can be of different types

##### Enum:
Each variant may have different fields or no fields



