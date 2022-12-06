use std::time::Instant;
use std::error::Error;
use crate::input_reader::read_file_in_cwd;

pub fn run() -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    
    let file = read_file_in_cwd("assets/input.txt");
    part_01(file);

    println!("Part 1 completed in: {:.2?}", started.elapsed());
    

    Ok(())
}

pub fn part_01(input:String) -> i32 {
    let lines:Vec<&str> = input.split("\n").collect();
    let mut one_contains_other_count: i32 = 0;
    
    for line in lines.iter() {
        let ranges:Vec<&str> = line.split(",").collect();
        if ranges.len() == 2 && one_contains_other(ranges[0], ranges[1]) {
            one_contains_other_count += 1;
            //println!("Added one to count: {},{},{}", ranges[0], ranges[1], one_contains_other_count);
        }        
    }    
    
    println!("Count one contains other: {}", one_contains_other_count);

    one_contains_other_count
}

pub fn one_contains_other(i1: &str, i2: &str) -> bool {
    let r1: Vec<i32> = i1.split("-").map(|s| { s.parse::<i32>().unwrap() }).collect();
    let r2: Vec<i32> = i2.split("-").map(|s| { s.parse::<i32>().unwrap() }).collect();


    
    (r1[0] >= r2[0] && r1[1] <= r2[1]) || (r1[0] <= r2[0] && r1[1] >= r2[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_contains_other__returns_true_if_contained_1(){
        let i1 = "6-6";
        let i2 = "4-6";

        let res = one_contains_other(i1,i2);

        assert_eq!(res, true)
    }

    #[test]
    fn one_contains_other__returns_true_if_contained_2(){
        let i1 = "2-8";
        let i2 = "3-7";

        let res = one_contains_other(i1,i2);

        assert_eq!(res, true)
    }

    #[test]
    fn one_contains_other__returns_false_if_contained(){
        let i1 = "1-10";
        let i2 = "2-11";

        let res = one_contains_other(i1,i2);

        assert_eq!(res, false)
    }

    #[test]
    fn one_contains_other__returns_false_if_contained_2(){
        let i1 = "3-7";
        let i2 = "7-43";

        let res = one_contains_other(i1,i2);

        assert_eq!(res, false)
    }

    #[test]
    fn part_01__works(){
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let res = part_01(input.to_string());

        assert_eq!(res, 2);
    }
}
