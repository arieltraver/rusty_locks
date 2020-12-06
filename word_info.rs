use std::fs;
//use std::io::prelude::*;
use std::string::String;
use std::collections::HashMap;
use std::str;
//use std::str::Chars;
//use std::slice;


fn main() {
    let test_1 = String::from("test_1.txt");
    let test_2 = String::from("test_2.txt");
    run_tests(test_1);
    run_tests(test_2);
}

fn prepare_buff(filename:String)-> String {
    let buff = fs::read_to_string(filename)
        .expect("bad file read");
    let mut newbuff = replace_chars(buff);
    newbuff = str::to_lowercase(&newbuff[..]);
    return newbuff;
}

fn hash_words(word_map:& mut HashMap<String, i32>, buff:&String, letter:&String) {
    // splits the word buffer by white spaces 
    // the method also creates a iterator
    let cursor = buff.split_whitespace();
    
    for current in cursor {
        // for each word in iterator that starts with the 
        if current.starts_with(letter) {
           if word_map.contains_key::<str>(&current) {
               let count=word_map.get_mut::<str>(&current).unwrap();
               *count = *count+1;
           }
           else { word_map.insert( (&current).to_string(), 1);
           }
        }
    }
} 
fn calculate_word_count(word_map: &mut HashMap<String,i32>,buff:&String){

    let alphabet = "abcdefjhijklmnopqrsuvwxyz".chars();

    for c in alphabet{
       //thread begins here 
      hash_words(word_map, &buff, &c.to_string());

    }

}

fn run_tests(filename:String){

    let buffer = prepare_buff(filename);
    println!("{}", buffer); 
    let mut word_map: HashMap<String,i32> = HashMap::new();  
    calculate_word_count(& mut word_map,&buffer);

    for ( word, count) in word_map.iter() {
        println!("the word count for {}: {}",word,count);
    }


}

fn replace_chars(buff:String) -> String {
    let v = vec![',','\"','.','!','?','(',')','{','}',':',';','。','，','\\', '/','_','“','”'];
    let mut new_buff = String::new();
    for ch in buff.chars() {
        if !v.contains(&ch) {
            new_buff.push(ch);
        }
        else if ch == '\n' {
            new_buff.push(' ');
        }
    }
    new_buff
} 
