use std::fs;
use std::cmp::Ordering;
use std::error::Error;

mod my_funcs;
mod helpers;
use crate::helpers::read_lines;

#[derive(Debug)]
pub struct Config {
    file_path: String
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    // let file_path = &args[1];
    // println!("Reading content from {}...", file_path);
    my_funcs::test_stuff();
    println!("{:?}", config);

    if let Ok(lines) = read_lines(config.file_path) {
        my_funcs::write_sums_to_vector(lines);
    }    

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
    

        let file_path:String = match args.len().cmp(&2) {
            Ordering::Less => "assets/input.txt".to_string(),
            Ordering::Equal => args[1].clone(),
            Ordering::Greater => return Err("argument count mismatch")
        };

        Ok(Self { file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){

    }
}