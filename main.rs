use std::fs::File;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::io;
use std::string::String;

struct Board<T: ?Sized> {
    b: T,
}

// define our 
#[derive(Default,Debug)]
struct Level {
    levelpack: i32,
    level_num: i32,
    width: i32,
    height: i32,
    num_colors: i32,
    //board: ;
}

impl Level {
    // Default constructor, could remove since Level has trait Default
    fn new(lp: i32, ln: i32, w: i32, h: i32, nc: i32) -> Level {
        Level {
            levelpack: lp,
            level_num: ln,
            width: w,
            height: h,
            num_colors: nc,
        }
    }
}

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

fn parse_header(level_header: &mut String) -> (i32, i32, i32, i32) {
    // Read in level header and convert to vector of ints
    let mut data: Vec<i32> = level_header.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    (data[0], data[1], data[2], data[3])
}

fn parse_level(level_str: &mut String, lp: i32, ln: i32) -> Level {
    // Take in level string and split into each section
    let mut data: Vec<String> = level_str.split(';').map(|l| l.to_string()).collect();

    // Get complete level metadata and store in Level
    let meta = parse_header(&mut data[0]);
    let mut level = Level { levelpack: lp, level_num: meta.2, ..Default::default() };

    // TODO: handle rectangle levels (for not assume square)
    level.height = meta.0;
    level.width = meta.0;
    level.num_colors = meta.3;

    level
}

fn main() {
    // Init
    let file: &str = "levels/levelpack_0.txt";
    let level_num: i32 = 100;
    // Read in level from levelpack
    let mut res = read_levelpack(file, level_num as usize).expect("Err: Couldn't find level/levelpack");
    let level = parse_level(&mut res.to_string(), 0, level_num);
    
    println!("{:?}", level);
}