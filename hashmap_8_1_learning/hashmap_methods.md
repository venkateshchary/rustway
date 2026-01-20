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

