### contains_key(&key)
- Check if key exists

```rust
if scores.contains_key("Blue") {
    println!("Blue team exists");
}
```
### get_mut("key")
```rust
let mut scores:HashMap<String, i32> = HashMap::new();

scores.insert(String::from("Blue"), 60);

match scores.get_mut("Blue"){
    Some(score) => *score+10,
    None => println!("key not found")
}
```

### entry('key')

case1:
If key not exists then create a key with value
```rust
use std::collections::HashMap;

let item_prices: HashMap<String, Vec<String>> = HashMap::new();
let mut mouse = "mouse".to_string();
item_prices.entry(mouse).or_insert(Vec::new());
// output :{"mouse" :[]}
println!("output: {:?}", item_prices);
```

case2:
if Key already exists then add new element to its value

```rust
item_prices.entry("mouse".to_string()).or_insert(Vec::new()).push("low".to_string());

println!("item_prices: {:?}", item_prices);
//item_prices: {"mouse": ["low"]}
```

case3:
If you wanted to modify the values of vector for particular key

```rust
item_prices.entry("mouse".to_string()).or_insert(Vec::new()).push("high".to_string()); // we inserted another element

// modify code
item_prices.entry("mouse".to_string()).and_modify(|values| 
    for each_value in values.iter_mut(){
        each_value.push_str(" too");
    }

)
println!("item_prices: {:?}", item_prices);
// item_prices: {"mouse": ["low too", "high too"]}

```


```scss

h.entry("key1".to_string())
// Entry<'_, String, Vec<String>>

.or_insert(Vec::new())
// &mut Vec<String>

.push("ve".to_string());
// ()
```
