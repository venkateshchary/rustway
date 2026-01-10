## Collections

Unlike array and tuple types, the data that these collections point to is stored on the heap.

Which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

### 1.Vectors
A vector allows you to store a variable number of values next to each other.

### 2. Hashmap
A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.


#### Vector in detail

##### How to access the elements

If you are sure about the index it exists then get by using the index
if you are not sure about the index then use the ``get``

```rust
let v: Vec<i32> = Vec![1,2,3,4];

let first_element = v[1];
// if you want get the last element suppose you know the length
let last_element = v[3];

// suppose you want to get the 100th element (i.e) it doesn't exist based on the declaration then you can use the get

let hundred_element = v.get(100);
// it will give Some(value) or None

// if we go with by index 100 it will raise an exception index out of bounds(panic)

```
While matching you can't use both statement(get and index)
i.e
```rust
if v1[0] == v1.get(2) {
        println!("both are equal");
    }else{
        println!("not equal");
    }
```
Because `get` will return `Option type` and index will return `int`


`v1.get(1)` return type is `Option<&i32>`

### collect

Suppose you wanted to create a `vector` with 10 elements starting from 1 to 10

2 ways you can achieve it 

1. using a for loop
2. using iterator with collect

**for loop**
```rust

let mut var_ten_number = Vec::new();

for i in 1..10{
var_ten_number.push(i);
}
```

**collect**
```rust
let numbers_ten:Vec<i32> = (0..10).collect();
```
>**Collect will take an iterator and transform into collection**

### Accessing the elements of Vector

While Iterating the elements you can't delete
Because `iter` gives immutable
```rust
for (index, value) in v_range.iter().enumerate(){
        println!("index = {:?}, value: {:?}",index,value);
        if value %2 ==0{
            println!("value is divisible by 2");
            v_range.remove(index); // won't work
        }
    }
```

##### iter() -- READ ONLY

- Borrows the collection **immutably**
- You can **read** the values
- You **can not modify the collection** or its elements

##### iter_mut() -- Modify elements, Not Structure
```rust
let mut v = vec![1, 2, 3];

for x in v.iter_mut() {
    *x *= 10;
}

println!("{:?}", v); // [10, 20, 30]
```
- Borrows the collection **mutably**
- You can **modify each element**
- You **can not change the vector length**

##### into_iter() -- Take Ownership
```rust
let v = vec![String::from("a"), String::from("b")];

for s in v.into_iter() {
    println!("{}", s); // s: String
}
// v is gone here ❌

```

- **Consumes** the collection
- Moves element out
- Original vector is **no longer usable**

##### retain() -- Filter in place

```rust
v.retain(|x| condition);

```
- It iterates safely
- Removes element **that do not match the condition**
- No borrow conflicts
- Modifies vector in place

#### remove(index)
    
- Removes and returns the element at the given index, shifting all elements after it to the left

#### pop()

- Removes and returns the last element as an Option<T>