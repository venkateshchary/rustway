
 ## Function available on Option
### unwrap_or
1. unwrap_or(default_value)
   💡 Meaning:
>“If Some(value) → give me the value.
> 
>If None → give me the default I provide.”

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// let sum = x + y;   // ❌ Not allowed

let sum = x + y.unwrap_or(0);
```

### unwrap_or_default
💡 Meaning:

>“If Some(value), give me the value.
> 
>If None, give me the default for the type using the Default trait.”
> 
>Each type has a built-in default:

| Like bool default is false | string default is "" | integer default  is 0 .. etc 

