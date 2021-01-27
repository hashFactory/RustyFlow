use std::fs::File;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::io;
use std::string::String;
//use std::result::Result;

fn read_levelpack(filename: &str, level_num: usize) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    //let level: Vec = vec!(text.split('\n').collect()?);
    match text.split('\n').nth(level_num) {
        None => Err(Error::new(ErrorKind::Other, "Could not find specified level")),
        Some(l) => Ok(l.to_string()),
    }
    /*
    match level.get(level_num) {
        Some(l) => (l.to_string()),
        _ => Err(Error::new(ErrorKind::Other, "Could not find specified level")),
    }*/
    //Ok(level.to_string())
}

fn main() {
    // Read in file
    let file: &str = "levels/levelpack_0.txt";
    let level_num: usize = 4;
    let res = read_levelpack(file, level_num).expect("Err: Couldn't find level");
    println!("{:?}", res);
}