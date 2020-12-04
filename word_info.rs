use std::fs;
//use std::io::prelude::*;
use std::string::String;
use std::collections::HashMap;
use std::str;
//use std::str::Chars;
//use std::slice;


fn main() {
    let filename = String::from("test_1.txt");
    let buffer = prepare_buff(filename);
    println!("{}", buffer); 
    let mut word_map: HashMap<String,i32> = HashMap::new();  
    hash_words(& mut word_map, &buffer, 'a');
    assert_eq!(word_map.get("apple"), Some(&1));
}

fn prepare_buff(filename:String)-> String {
    let buff = fs::read_to_string(filename)
        .expect("bad file read");
    let newbuff = buff.replace("\n"," ");
    return newbuff;
}

fn hash_words(word_map:& mut HashMap<String, i32>, buff:&String, letter:char) {
    let cursor = buff.split_whitespace();
    //let current = cursor.next().unwrap();
    // use an iterator
    for current in cursor {
       if current.contains(letter) {
           if word_map.contains_key::<str>(&current) {
               let count=word_map.get_mut::<str>(&current).unwrap();
               *count = *count+1;
           }
           else { word_map.insert( (&current).to_string(), 1);
           }
        }
    }
}

