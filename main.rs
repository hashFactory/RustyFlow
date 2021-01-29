#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

use std::fs::File;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::io;
use std::string::String;
use std::slice::Iter;

use self::Direction::*;

#[derive(Debug,PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }
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
    endpoints: Vec<(u8, u8)>,
    //board: &'a Board<u32>,
}

#[derive(Default,Debug,Clone)]
struct Node {
    parent: u32,
    id: u32,
    val: u8,
    dir: u8,
    color: u8,
    b: Vec<u8>,
    tainted: [u8; 4],
    possible: [u32; 4],
}

struct Moves {
    all: Vec<Node>,
    root: Option<Node>
}

impl Level {
    // Default constructor, could remove since Level has trait Default
    fn new(lp: u8, ln: u8, w: u8, h: u8, nc: u8, bb: Vec<u8>, ep: Vec<(u8, u8)>) -> Level {
        Level {
            levelpack: lp,
            level_num: ln,
            width: w,
            height: h,
            num_colors: nc,
            b: bb,
            endpoints: ep,
        }
    }
}

fn play(b: &mut Vec<u8>, new_val: u8, color: u8) -> Vec<u8>{
    b[new_val as usize] = color;
    b.to_vec()
}

impl Node {
    fn new(id: u32, v: u8, d: u8) -> Node {
        Node {
            id,
            val: v,
            dir: d,
            ..Default::default()
        }
    }

    pub fn apply(&self, dir: u8, id: u32, new_val: u8) -> Node {
        let new_b: &mut Vec<u8> = &mut self.b.clone();
        new_b[new_val as usize] = self.color;
        Node {
            parent: self.id,
            id,
            dir,
            val: new_val,
            tainted: [0; 4],
            possible: [0; 4],
            b: new_b.to_vec(),
            color: self.color
        }
    }
}

fn populate_from_endpoints(b: &mut [u8], ep: &[(u8, u8)]) {
    // Given a brand new board, populate it with endpoints we know have to be there
    let mut i = 1;
    for c in ep {
        b[c.0 as usize] = i;
        b[c.1 as usize] = i;
        i += 1;
    }
}

fn repr_board(b: &[u8], w: u8) -> String {
    // Setup
    let mut res = String::from("");
    let size = b.len() / (w as usize);
    // For each row and line add to res: String the value at that spot
    // TODO: better handle when double digits, maybe color code
    for i in 0..size {
        for j in 0..w { res += &(b[(w * (i as u8) + j) as usize].to_string() + " ") }
        res += "\n";
    }
    res
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
    //println!("{:?}", level.b);

    // Fetch coordinates out from the solution in levelpack.txt
    for p in paths_str {
        // Fetch coordinates out from the solution in levelpack.txt
        let path = p.split(',').map(|v| v.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        // Populate level solution
        level.endpoints.push((path[0], path[path.len() - 1]));
        for coord in path { level.b[coord as usize] = count }
        count += 1;
    }

    println!("{}", repr_board(&level.b, level.width));
}

fn check_if_possible(node: &Node, dir: u8, width: u8) -> u8 {
    use Direction::*;
    // If came from same direction, don't allow
    if dir == node.dir { return u8::MAX }

    // Find where travelling certain direction would take it
    let new_val = match dir {
        0 => if node.val >= width { node.val - width } else { return u8::MAX },
        1 => if node.val + width < node.b.len() as u8 { node.val + width } else { return u8::MAX },
        2 => if node.val % width > 0 { node.val - 1 } else { return u8::MAX },
        3 => if node.val % width < width - 1 { node.val + 1 } else { return u8::MAX },
        _ => return u8::MAX
    };
    // Check if new spot is occupied or not
    if node.b[new_val as usize] != 0 { return u8::MAX }
    new_val as u8
}

fn create_tree_for_color(root: &mut Node, level: &Level, moves: &mut Moves, id: &mut u32) -> () {
    // Root contains a board with currently established board
    for dir in 0..4 {
        root.possible[dir as usize] = match check_if_possible(&root, dir, level.width) {
            u8::MAX => u32::MAX,
            new_val => { *id += 1;
                moves.all.append(root.apply(dir, id, new_val));
                println!("{:?}", moves.all);
                id
            }
        };
    }
}

fn start_run(level: &Level) {
    let mut b: Vec<u8> = vec![0; (level.width * level.height) as usize];
    populate_from_endpoints(&mut b[..], &level.endpoints);
    println!("{}", repr_board(&b, 5));

    let mut moves = Moves { all: Vec::new(), root: None };
    let mut root: Node = Node::new(0, level.endpoints[0].0, 5);
    root.color = 1;
    root.parent = 0;
    root.b = b;
    root.id = 0;
    moves.all.append(root.clone());

    let mut queue: Vec<u32> = vec![0];
    let mut id: u32 = 0;

    while (queue.len() > 0) {
        create_tree_for_color(&mut moves.all[queue.pop() as usize], &level, &mut moves, &mut id);
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
    level.endpoints = Vec::<(u8, u8)>::new();

    // Fill solutions from levelpack.txt
    parse_paths(&data[1..], &mut level);
    //println!("{:?}", level.b);

    level
}

fn main() {
    // Init
    let file: &str = "levels/levelpack_0.txt";
    let level_num: u8 = 0;
    // Read in level from levelpack
    let res = read_levelpack(file, level_num as usize).expect("Err: Couldn't find level/levelpack");
    let level = parse_level(&mut res.to_string(), 0);
    start_run(&level);
    
    println!("{:?}", level);
}