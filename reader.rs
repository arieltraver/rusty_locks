use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    File::create("example.txt")?;
    let mut example = File::create("example.txt")?;
    let mut example2 = File::open("example.txt")?;
    let contents = b"this is a file \n";
    example.write_all(contents)?;
    let mut results = String::new();
    example2.read_to_string(&mut results)?;
    println!("{}", results);
    Ok(())
}


