use std::process;
use std::num::ParseIntError;
use std::fs::File;
use std::io::{ self, BufRead };

use crate::helpers::read_lines;


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

pub fn get_max(lines: io::Lines<io::BufReader<File>>, top_count:i32) -> i32 {
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

    sums.sort();
    let mut sum = 0;
    for i in 0..top_count {
        let current_value = sums[sums.len()-1-(i as usize)];
        println!("Current value beeing added: {:?}", current_value );
        sum = sum + current_value;
    }
    println!("Maximum value: {:?}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_or_sum__number_should_be_added() {
        let current:i32 = 15;
        let line:&str = "1234";

        let result = count_or_sum(line, current).unwrap();

        assert_eq!(result, 1249);
    }

    #[test]
    fn count_or_sum__number_should_remain_the_same_with_empty_line(){
        let current:i32 = 142;
        let line:&str = "";

        let result = count_or_sum(line, current).unwrap();

        assert_eq!(result, 142);
    }

    #[test]
    fn write_sums_to_vector__simple_block_works(){
        let test_file = read_lines("test/my_funcs/write_sums_to_vector_01.txt").unwrap();

        let max = get_max(test_file);

        assert_eq!(max, 15);

    }
}