use std::fs::File;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::io;
use std::string::String;

fn read_levelpack(filename: &str, level_num: usize) -> io::Result<String> {
    // Read in entire file to mut string
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    // Find nth level(line), if not found, throw error
    match text.split('\n').nth(level_num) {
        None => Err(Error::new(ErrorKind::Other, "Could not find specified level")),
        Some(l) => Ok(l.to_string()),
    }
}

fn main() {
    // Init
    let file: &str = "levels/levelpack_0.txt";
    let level_num: usize = 100;
    // Read in level from levelpack
    let mut res = read_levelpack(file, level_num).expect("Err: Couldn't find level/levelpack");
    
    println!("{:?}", res);
}