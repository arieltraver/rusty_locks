use std::collections::HashMap;

fn main() {
    let mut cats = HashMap::new(); //infer: HashMap<String, String>
    cats.insert("Best Cat".to_string(), "Fluffy".to_string()); //now has a name in it
    cats.insert("Evil Cat".to_string(), "Segfault".to_string());

    if cats.contains_key("Best Cat") {
        println!("Best cat is in the house");
    }
    else {println!("No good kitties here!");}
    
    for (title, name) in &cats {
        println!("{}: {}", title, name);
    }


}

