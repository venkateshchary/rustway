### Match

As we know for the match we need to provide all arm variants

Means

Take an example of Option
It is having 2 variants(Some & None)

If any variable is declared with Option<T> type
then doing a match we need to provide the 2 arms

```rust
fn main(){
    
    let gender :Option<String> = Some("M");
    
    match gender(g) {
        some(g)=>{
            println!("gender is: {:?}", gender);
        },
        None => {
            println!("undefined the gender");
        }
    }
    // Suppose you don't want to declare all arms. Only particular arm only needed for your case
    // then we use if let
    // or
    // When you only care about one pattern and want to ignore the rest.
    
    /*
        if let version
     */
        if let Some(g)= gender{
            println!("value is :{:?}", g);
        }
}
```

#### match

- Must cover all variants.
- Good when you need to handle every case.

#### if let

- Only checks one pattern.
- Ignores everything else.
- Good when you only care about one case.