

### Data types
1. Integer(i32,i64)
2. Float(f32,f64)
3. String
4. Char
5. Bool
6. Tuple
7. Arrays
8. Vector
9. Enum/Struct (custom data types)

### which data types store the data in stack memory

1. Char
2. Bool
3. Integer
4. Float
5. Tuple
6. Arrays

### Which data types store the data in heap memory
1. String(both heap and stack)
2. Vector
3. Enum/Struct

### By default copy is implemented for below data types
1. integer
2. float
3. char
4. bool

> all these are owns the memory in the stack


### Rule of thumb for copy trait

> A data type is copy `if and only if all of its parts are copy`

### Can be made `Copy`
1. Enum with no heap data stored data types
```rust
enum IpAddress{
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddress,
    value: (u8,u8,u8,u8),
}

let v4_type = IpAddress::V4;
let ip1 = IpAddr{
    kind: v4_type,
    value: (127,0,0,1)
}
println!("ip1: {:?}", ip1);

```

---

## Why `String` not `copy`?
Becuase copying a string would mean duplicating
- a pointer
- length
- capacity
- and heap data

That is expensive and unsafe todo implicitly
