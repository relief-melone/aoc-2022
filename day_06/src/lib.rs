mod input_reader;
mod part_01;
mod part_02;

use std::collections::VecDeque;

pub fn run(){

    let input = input_reader::read_file_in_cwd("assets/input.txt");

    part_01::run(input.clone()).unwrap();
    part_02::run(input.clone()).unwrap();
    
}

pub fn find_marker(input:String, unique_seq_count: i32) -> (i32, String) {
    let mut last_four = VecDeque::<char>::new();

    for (ind, char) in input.chars().enumerate() {

        last_four.push_back(char);
        if last_four.len() < unique_seq_count as usize {
            continue;
        } else {
            let mut all_chars_unique = true;
            
            for char in last_four.iter(){
                if last_four.iter().filter(| &c | *c == *char).count() > 1  {
                    all_chars_unique = false;
                };                
            };

            if all_chars_unique {
                let mut res = String::new();
                for char in last_four.iter(){
                    res.push(*char)
                }
                return (ind as i32+1, res);
            }

            last_four.pop_front();
        }
    }

    (-1, "".to_string())
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn find_marker_01(){
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let (index, marker) = find_marker(input, 4);

        assert_eq!(marker, "jpqm");
        assert_eq!(index, 7);
    }

    #[test]
    fn find_marker_02(){
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let (index, marker) = find_marker(input, 4);

        assert_eq!(marker, "vwbj");
        assert_eq!(index, 5);
    }

    #[test]
    fn find_marker_03(){
        let input = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let (index, marker) = find_marker(input, 4);

        assert_eq!(marker, "pdvj");
        assert_eq!(index, 6);
    }

    #[test]
    fn find_marker_04(){
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let (index, marker) = find_marker(input, 14);

        assert_eq!(marker, "qmgbljsphdztnv");
        assert_eq!(index, 19);
    }
}