mod input_reader;
mod part_01;
mod part_02;

use std::collections::VecDeque;

pub fn run(){

    let input = input_reader::read_file_in_cwd("assets/input.txt");

    part_01::run(input.clone()).unwrap();
    part_02::run(input.clone()).unwrap();
    
}


#[cfg(test)]

mod test {
    use super::*;

  
}