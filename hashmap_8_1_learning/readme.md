### HashMap
The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function

> Rust doesn't automatically import Hashmap
> Due to this we need import manually unlike `vector`

Reasons:
1. More specified
2. Heavier than ``Vec``
3. Not used in every program

So Rust **does not auto import it**

#### Why this one fails
```rust
// using unwrap you will get actual score type i32
println!("jack_student exam scores: {:?}", students.get(&jack_student).unwrap_or_default()); //42
```
Hash Map::get(key) will give output type of `Option<&i32>`. But the unwrap input type is required `Option<i32>`

As we know `Option<i32>` default value is 0

It is **unable to get the default value for the reference (&i32) due to this it is failing**

#### Copied()
Copied() convert `Option<&i32>` to `Option<i32>`

On top of the copied() we can use the unwrap_or_default()


#### Borrowed
```rust
let mut map:HashMap<String, String> = HashMap::new();
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

map.insert(field_name, field_value);
println!("map: {:?}", map);
// println!("field_value: {:?}", field_value); this will give error like 'value is borrowed here after move'
```
> You can't use the field_name & field_value variable , Because the values are moved to HashMap

**You can Avoid this By following any of below approach**
1. Initialize the HashMap like `HashMap<&String, &String>`
2. Use the string literals `&'static str`
3. Use the field_name.clone() , field_value.clone()

[methods](hashmap_methods.md)