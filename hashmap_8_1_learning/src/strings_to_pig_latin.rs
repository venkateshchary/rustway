/*
Convert strings to Pig Latin.
 The first consonant of each word is moved to the end of the word and ay is added,
  so first becomes irst-fay.
  Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
 Keep in mind the details about UTF-8 encoding!
 */
use std::str::Chars;

pub fn convert_to_pig_latin(phrase: &str) {

    let vowels:Vec<char> = vec!['a','e', 'i', 'o', 'u'];
    let mut result_output: String =  String::from("");
    let hypen:char = '-';
    let vowels_suffix:String = "-hay".to_string();
    let consonant_suffix:String = "ay".to_string();
    for each_word in phrase.split(" ") {
        println!("each word: {:?}", each_word);
        let mut consonants_char_found: bool = false;
        let mut string_formatter: String = String::from("");
        let mut first_char = "".to_string();
        for (index, char) in each_word.chars().enumerate(){

            if consonants_char_found{
                println!("consonants_char_found: {:?}", char);
                let without_first_letter: String = each_word.chars().skip(1).collect();
                result_output.push_str(&without_first_letter);
                result_output.push(hypen);
                result_output= result_output+&first_char+&consonant_suffix+" ";
                break
            }
            if index ==0 {
                println!("at index 0");
                consonants_char_found = false;
                if vowels.contains(&char){
                    string_formatter = string_formatter+each_word+&vowels_suffix+" ";
                    println!("before break: {:?}",string_formatter);
                    result_output += string_formatter.as_str();
                }else {
                    // consonant_suffix += char;
                    println!(" consonant--{:?}", each_word);
                    first_char.push(char);
                    consonants_char_found= true;
                }
            }
        }
    }
    let final_result_output = result_output.trim_end();
    println!("result output: {:?}",final_result_output);
}

pub fn better_way_computing_pig_latin(phrase: &str) {
    let mut result_str :String = String::from("");
    for word in phrase.split_whitespace(){
        println!("word: {:?}", word);

        let mut chars: Chars = word.chars(); // iterator
        println!("chars: {:?}", chars);
        // println!("each time 1 character : {:?}", chars.next());
        // println!("each time 1 character  {:?}", chars.next());
        // println!("each time 1 character {:?}", chars.next());
        /*
        chars: Chars(['p', 'r', 'o', 'g', 'r', 'a', 'm', 's'])
        each time 1 character : Some('p')
        each time 1 character  Some('r')
        each time 1 character Some('o')
         */
        match chars.next(){
            Some('a') => {
               println!("vowels: a");
                result_str.push_str(&word);
                result_str.push_str(&"-hay".to_string());
                result_str.push_str(&" ".to_string());
            }
            Some('e') => {
                println!("vowels: e");
                result_str.push_str(&word);
                result_str.push_str(&"-hay".to_string());
            }
            Some('i') => {
                println!("vowels: i");
                result_str.push_str(&word);
                result_str.push_str(&"-hay".to_string());
                result_str.push_str(&" ".to_string());
            }
            Some('u') => {
                println!("vowels: u");
                result_str.push_str(&word);
                result_str.push_str(&"-hay".to_string());
                result_str.push_str(&" ".to_string());
            }
            Some('o') => {
                println!("vowels: o");
                result_str.push_str(&word);
                result_str.push_str(&"-hay".to_string());
                result_str.push_str(&" ".to_string());
            }
            other => {
                println!("consonant word: {:?}", word);
            }
        }
        println!("result: {:?}", result_str);

    }
}

pub fn combined_logic_computing_pig_latin(phrase: &str) -> String {
    let mut result_str :String = String::new();
    for word in phrase.split_whitespace(){
        println!("word: {:?}", word);
        let mut chars: Chars = word.chars();
        println!("chars: {:?}", chars);
        match chars.next() {
            Some('a') | Some('e') | Some('i') | Some('u') | Some('o') => {
                println!("vowels: e");
                result_str.push_str(word);
                result_str.push_str("-hay");
                result_str.push(' ');
            }
            Some(first_char) => {
                println!("consonant--{:?}", first_char);
                // first char already moved from chars because we used the next
                // so remaining will convert to string
                // collect will take an iterator and convert it into collection
                let rest: String = chars.collect();
                println!("rest: {:?}", rest);
                result_str.push_str(&rest);
                result_str.push('-');
                result_str.push(first_char);
                result_str.push_str("ay");
                result_str.push(' ');
            }
            None => {
                println!("---");
            }
        }
    }
    result_str
}