use std::path::Path;
use std::io::{ self, BufRead };
use std::fs::File;
use std::env;
use std::cmp::Ordering;

#[allow(dead_code)]

pub fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Config {
    pub part: i32
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        
        let part:i32 = match args.len().cmp(&2) {
            Ordering::Less => 1,
            Ordering::Equal => args[1].clone().parse().unwrap(),
            Ordering::Greater => panic!("argument count mismatch")
        };

        Self { part }
    }
}