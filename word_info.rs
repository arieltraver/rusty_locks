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
    let newbuff = buff.replace("\n"," ");
    return newbuff;
}

fn hash_words(word_map:& mut HashMap<String, i32>, buff:&String, letter:char) {
    // splits the word buffer by white spaces 
    // the method also creates a iterator
    let cursor = buff.split_whitespace();
    //let current = cursor.next().unwrap();
    
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
} fn calculate_word_count(word_map: &mut HashMap<String,i32>,buff:&String){

    let alphabet = "abcdefjhijklmnopqrsuvwxyzABCDEFJHIJKLMOPQRSTUVXYZ".chars();

    for c in alphabet{
        
      hash_words(word_map, &buff, c);

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

