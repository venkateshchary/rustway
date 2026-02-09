
```scss
rror[E0277]: the trait bound `list_users_reader::User: serde::Deserialize<'de>` is not satisfied
    --> src\list_users_reader.rs:50:27
     |
50   |     let data: Vec<User> = serde_json::from_str(&file_string_data)?;
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `list_users_reader::User`
```