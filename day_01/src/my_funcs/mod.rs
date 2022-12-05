use std::process;
use std::num::ParseIntError;
use std::fs::File;
use std::io::{ self, BufRead };



use crate::helpers::read_lines;

pub fn test_stuff(){
    println!("Hello from funcs");
}

pub fn count_or_sum(line: &str, current: i32) -> Result<i32, ParseIntError> {
    if line.is_empty(){
        return Ok(current);
    }
        

    let num = match line.parse::<i32>() {
        Ok(number) => Ok( number+ current ),
        Err(err) => Err(err)
    };

    num
}

pub fn write_sums_to_vector(lines: io::Lines<io::BufReader<File>>){
    let mut sums = Vec::<i32>::new();
    let mut current = 0;
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {                
                sums.push(current);
                current = 0;
            } else {
                current = count_or_sum(&line, current).unwrap();
            }            
        }
    }

    let max = sums.iter().max();
    println!("{:?}", max.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_should_be_added() {
        let current:i32 = 15;
        let line:&str = "1234";

        let result = count_or_sum(line, current).unwrap();

        assert_eq!(result, 1249);
    }

    #[test]
    fn number_should_remain_the_same_with_empty_line(){
        let current:i32 = 142;
        let line:&str = "";

        let result = count_or_sum(line, current).unwrap();

        assert_eq!(result, 142);
    }
}