use std::collections::HashMap;

pub fn text_splitter_to_hashmap() -> HashMap<String, u32> {
    let phrase: String = String::from("shows some code that counts how many times each word appears in some text");
    let mut words_count: HashMap<String, u32> = HashMap::new();
    for word in phrase.split(" ") {
        println!("{:?}", word);
        words_count.entry(word.to_string()).and_modify(|e| *e += 1).or_insert(1);
    };
    words_count

}