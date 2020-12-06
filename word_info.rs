use std::fs;
use std::thread;
use std::string::String;
use std::collections::HashMap;
use std::str;
use std::collections::LinkedList;
//use std::str::Chars;
//use std::slice;


struct LetterMap{
    letter: String,
    map: HashMap<String, i32>,
    key_list: LinkedList<String>,
}

 fn main() {
    let test_1 = String::from("test_1.txt");
    let test_2 = String::from("test_2.txt");
    run_tests(test_1);
    run_tests(test_2);
}

fn prepare_buff(filename:String) -> String{
    let mut buff:String = fs::read_to_string(filename)
        .expect("bad file read");
    replace_chars(buff);
    buff = str::to_lowercase(&buff[..]);
    buff
}

fn hash_words(letter_struct:LetterMap, letter:&String, buff:&String) {
    // splits the word buffer by white spaces 
    // the method also creates a iterator
    let cursor = buff.split_whitespace();
    
    for current in cursor {
        // for each word in iterator that starts with the 
        if current.starts_with(letter) {
           if letter_struct.map.contains_key::<str>(&current) {
               let count=letter_struct.map.get_mut::<str>(&current).unwrap();
               *count = *count+1;
           }
           else { letter_struct.map.insert( (&current).to_string(), 1);
           }
        }
    }
} 
fn calculate_word_count(buff:&String)->Vec<LetterMap>{

    let alphabet = "abcdefjhijklmnopqrsuvwxyz".chars();
    let alphabet2 = "abcdefjhijklmnopqrsuvwxyz".chars();
    let mut hash_vec = Vec::with_capacity(26);
    for ch in alphabet2 {
        let mut chs = ch.to_string();
        let mut hmap = HashMap::<String,i32>::new();
        let mut llist = LinkedList::<String>::new();
        let mut lmap = LetterMap{letter:chs,map:hmap,key_list:llist};
        hash_vec.push(lmap); //fill this vector with a struct for each letter
    }
    let mut i = 0;
    for c in alphabet{
        thread::spawn(move||{ //make a new thread for every letter of the alphabet
            hash_words(hash_vec[i], &c.to_string(), buff);
        });
        i+=1;
    }
    hash_vec
}

 fn run_tests(filename:String){

    let buff:String = prepare_buff(filename);
    println!("{}", buff); 
    let v = calculate_word_count(&buff);

    for lmap in v {
        for (word, count) in lmap.map.iter() {
            println!("the word count for {}: {}",word,count);
        }
    }

}

fn replace_chars(buff:String)->String {
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
