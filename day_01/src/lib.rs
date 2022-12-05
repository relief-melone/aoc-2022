use std::fs;
use std::cmp::Ordering;
use std::error::Error;

mod funcs;
mod helpers;
use crate::helpers::read_lines;

#[derive(Debug)]
pub struct Config {
    part: i32,
    file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
    
        let file_path = "assets/input.txt".to_string()        
;
        let part:i32 = match args.len().cmp(&2) {
            Ordering::Less => 1,
            Ordering::Equal => args[1].clone().parse().unwrap(),
            Ordering::Greater => return Err("argument count mismatch")
        };

        Ok(Self { file_path, part })
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    if config.part == 1{
        run_part_01(config)
    } else {
        run_part_02(config)
    }
}

pub fn run_part_01(config:Config) -> Result<(), Box<dyn Error>>{
    println!("Running Part 01");

    if let Ok(lines) = read_lines(config.file_path) {
        funcs::get_max(lines, 1);
    }    

    Ok(())
}

pub fn run_part_02(config:Config) -> Result<(), Box<dyn Error>> {
    println!("Running Part 02");
    if let Ok(lines) = read_lines(config.file_path) {
        funcs::get_max(lines, 3);
    }

 Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){

    }
}