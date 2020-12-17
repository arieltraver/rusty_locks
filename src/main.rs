#[macro_use]
extern crate lazy_static;
use std::time::{Duration, Instant};
use std::fs;
use std::string::String;
use std::str;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Arc;
//use fnv::FnvHashMap; //we tested another hash function but it didn't do much
use std::hash::{Hash, Hasher};
use std::collections::HashMap;


// creates a global instance of the text which can be shared with threads
lazy_static! {
    pub static ref TEXT_LOCK: RwLock<String> = RwLock::new(String::new());
    pub static ref RESULT_VECTOR: Mutex<Vec<(String, i32)>> = Mutex::new(vec![]); 
}

///main
///just calls run_tests
fn main() {
    run_tests();
}

/// run_tests
/// runs a set of tests on different inputs and using different numbers of threads
 fn run_tests() {
    
    let test_1 = String::from("./text/3randstrnonum.txt");
    prepare_buff(test_1);
   
    //fill the cache so that isn't included in our calculation
    //we noticed first time for all test cases is often slower
    calculate_word_count(7, true);

    let start = Instant::now();
    calculate_word_count(7, true);
    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}", elapsed);
}

/// prepare_buff
/// Clean the text from the file, replacing punctuation marks and newlines with spaces
/// Store the text in a lazy static String
/// # Arguments
/// * filename: the name or filepath of the text
fn prepare_buff(filename:String){
    let mut buff:String = fs::read_to_string(filename)
        .expect("bad file read");
    buff = replace_chars(buff);
    buff = str::to_lowercase(&buff[..]); 
    let mut text = TEXT_LOCK.write().unwrap(); //unlock the static String
    *text = buff;
}

///calculate_word_single
///Iterates through each word in the text and places it in its appropriate position in the hashmap.
///This function uses no threads.
fn calculate_word_single() {
    //let mut map = FnvHashMap::with_hasher(Default::default()); //alternate hasher
    let mut map = HashMap::<String, i32>::new(); //generate a hash table
    let buff = TEXT_LOCK.read().unwrap(); //unlock for reading
    let cursor = buff.split_whitespace(); //generate iterator
    for current in cursor {
   		if map.contains_key::<str>(&current) {
            let count=map.get_mut::<str>(&current).unwrap(); //value at key
            *count = *count+1
			}
        else {map.insert((&current).to_string(), 1);
    	}
    } 
}

/// calculate_word_count
/// Spawns threads for each range of starting letters
/// Each thread has its own hash table.
/// Threads print out their results once their calculations are complete
/// # Arguments
///     * range0: the range of u8 (unicode) starting letters per thread
///     * extras: boolean indicating if words start with non-alphabetical characters are included
fn calculate_word_count(range0:u8, mut extras:bool){
    let mut i:u8 = 97; //u8 for the character 'a'
    crossbeam::scope(|s| { //threads guaranteed to join before this scope ends
    while i <= 122 { //u8 for the character 'z'
        let icopy = Arc::new(i); //u
        let range = Arc::new(range0);
        let extras_not_done = Arc::new(extras);
        s.spawn(move |_| { //spawn a thread and move variables in
            let mut hmap = HashMap::<String,i32,>::new(); //new map for thread
            //let mut hmap = FnvHashMap::with_hasher(Default::default());
            let buff = TEXT_LOCK.read().unwrap(); // acquire the lock for reading
            let cursor = buff.split_whitespace(); //break by spaces
            for current in cursor {
                let first = current.chars().next().unwrap() as u8;
                //the extras_not_done determines whether random non-letters should be counted
                if (first >= *icopy && first < *icopy + *range ) 
                   | (*extras_not_done && (first < 97 || first > 122))
                { //only look at certain letters
                    if hmap.contains_key::<str>(&current) {
                        let count=hmap.get_mut::<str>(&current).unwrap();
                        *count = *count+1; // increment the count
                    }
                    else { hmap.insert((&current).to_string(), 1); // insert new entry into hashmap
                    }
                }
            }
            // let mut vector = RESULT_VECTOR.lock().unwrap(); // acquire the lock for reading
            // /* for (key, count) in &hmap {
            //     println!("{} {}", key, count); // printing the hashmap
            //     let mut key2 = key.clone();
            // vector.push((key2,*count)); //adding to the tuple
            // } */
        });
        extras = false; // toggle false
        i += range0;
    }
}).unwrap();
    analyze_results();
}

/// replace_chars
/// Cleans out pesky punctuation and newlines
/// # Arguments
///     * buff: the String to be cleaned
/// # Returns
///     * newbuff: a copy of the original without punctuation except spaces
fn replace_chars(buff:String)->String {
    // define the punctuations that we wish to removed
    let v = vec![',','\"','.','!','?','(',')','{','}',':',';','。','，','\\', '/','_','“','”', '|', '+', '–'];
    let mut new_buff = String::new(); // create new String to store cleaned buff
    let iterator = buff.chars(); // creates an iterator over chars
    for ch in iterator {
        if v.contains(&ch) {
            new_buff.push(' '); //don't push punctuation in
        }
        else if ch == '\n' {
            new_buff.push(' ');
        }
        else {
            new_buff.push(ch); // push char in
        }
    }
    new_buff // return the cleaned string
}

///analyze_results
///Takes the vector of tuples(word, count) from the counter and sorts it.
///After sorting, it prints the results in order.
fn analyze_results() {
    let mut vec = RESULT_VECTOR.lock().unwrap(); // acquire the lock for writing
    &(*vec).sort_by(|a, b| a.0.clone().cmp(&b.0.clone())); // sort the vector by lambda function
    for (key, count) in &*vec {
        println!("The word count for {}:{}", key, count); // printing the sorted tuple
    }
    *vec = vec![]; // cleaning the tuple
}
