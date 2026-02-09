## Serialize

convert `Rust data` -> some format

If a type implements `Serialize`, it **can be converted OUT**

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
struct User{
    first_name: String,
    age: u8,
}

fn main(){
    let u1 = User{first_name: "venkatesh".to_string(), age:32};
    println!("user 1: {:#?}", u1);
    println!("----- serializing-----");
    
}
```

## Deserialize

convert some format -> **Rust Data**

- **Deserialize** is a trait
- If type implements `Deserialize`, it **can be created From data**
```scss
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
name: String,
age: u8,
}
```

## Serializer

A ``serializer`` is the thing **that knows how to write data in a specific format.**

EG:
- JSON serializer
- YAML serializer
- TOML serializer

```rust
serde_json::to_string(&user1);

```
- ``serde_json`` Providers a **JSON Serializer**
-  It walks through your struct field-by-field
- Produces a JSON string


Flow:
```scss
User
  ↓ (Serialize trait)
serde_json::Serializer
  ↓
JSON string

```

## Deserializer

A **Deserializer** knows how to read data from a specific format

```rust
serde_json::from_str::<User>(json_str)
```

Flow:
```scss
JSON string
  ↓
serde_json::Deserializer
  ↓ (Deserialize trait)
User

```



### users_age_string

```json
{ "name": "John Doe", "age": "30" }
```
use case:

Suppose instead of age is an integer in one of the file we received it as string
