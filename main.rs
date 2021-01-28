#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

use std::fs::File;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::io;
use std::string::String;
use std::env;

// TODO: figure out how I'm going to store the board
struct Board<T: ?Sized> {
    b: T,
}

// Define the level struct
#[derive(Default,Debug)]
struct Level {
    levelpack: u8,
    level_num: u8,
    width: u8,
    height: u8,
    num_colors: u8,
    b: Vec<u8>,
    //board: &'a Board<u32>,
}

impl Level {
    // Default constructor, could remove since Level has trait Default
    fn new(lp: u8, ln: u8, w: u8, h: u8, nc: u8, bb: Vec<u8>) -> Level {
        Level {
            levelpack: lp,
            level_num: ln,
            width: w,
            height: h,
            num_colors: nc,
            b: bb,
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

fn parse_header(level_header: &mut String) -> (u8, u8, u8, u8) {
    // Read in level header and convert to vector of ints
    let mut data: Vec<u8> = level_header.split(',').map(|v| v.parse::<u8>().unwrap()).collect();
    (data[0], data[1], data[2], data[3])
}

fn parse_paths(paths_str: &[String], level: &mut Level) -> () {
    // Counter will keep track of which color we're on
    let mut count: u8 = 1;
    println!("{:?}", level.b);

    // Fetch coordinates out from the solution in levelpack.txt
    for p in paths_str {
        // Fetch coordinates out from the solution in levelpack.txt
        let path = p.split(',').map(|v| v.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        // Populate level solution
        for coord in path { level.b[coord as usize] = count }
        count += 1;
    }
}

fn parse_level(level_str: &mut String, lp: u8) -> Level {
    // Take in level string and split into each section
    let mut data: Vec<String> = level_str.split(';').map(|l| l.trim().to_string()).collect();

    // Get complete level metadata and store in Level
    let meta = parse_header(&mut data[0]);
    let mut level = Level { levelpack: lp, level_num: meta.2, ..Default::default() };

    // TODO: handle rectangle levels (for not assume square)
    level.height = meta.0;
    level.width = meta.0;
    level.num_colors = meta.3;
    level.b = vec![0; (level.height * level.width) as usize];

    // Fill solutions from levelpack.txt
    parse_paths(&data[1..], &mut level);
    //println!("{:?}", level.b);

    level
}

fn main() {
    // Init
    let file: &str = "levels/levelpack_0.txt";
    let level_num: u8 = 100;
    // Read in level from levelpack
    let res = read_levelpack(file, level_num as usize).expect("Err: Couldn't find level/levelpack");
    let level = parse_level(&mut res.to_string(), 0);
    
    println!("{:?}", level);
}