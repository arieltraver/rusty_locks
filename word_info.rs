use std::fs;
//use std::io::prelude::*;
use std::string::String;
use std::collections::HashMap;
use std::str;
use std::str::Chars;
use std::slice;


fn main() {
    let filename = String::from("test_1.txt");
    let buffer = prepare_buff(filename);
    println!("{}", buffer); 
    let mut word_map: HashMap<&str,i32> = HashMap::new();  
    hash_words(&word_map, buffer, 'a');
}

fn prepare_buff(filename:String)-> String {
    let buff = fs::read_to_string(filename)
        .expect("bad file read");
    let newbuff = buff.replace("\n"," ");
    return newbuff;
}

fn hash_words(word_map:&HashMap<&str, i32>, buff:String, letter:char) {
    let mut cursor = buff.split_whitespace();
    let current = cursor.next();
    while current != None {
       if current.chars().next() == letter {
           if word_map.contains_key(current) {
               word_map.insert(word_map.get(current) + 1);
           }
           else { word_map.insert(current, 1);
           }
        }
    }
}

