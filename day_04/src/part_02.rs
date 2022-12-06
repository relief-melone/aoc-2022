use std::time::Instant;
use std::error::Error;
use crate::input_reader::read_file_in_cwd;


pub fn run() -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    
    let file = read_file_in_cwd("assets/input.txt");
    part_02(file);

    println!("Part 2 completed in: {:.2?}", started.elapsed());
    

    Ok(())
}

pub fn part_02(input:String) -> i32 {
    let lines:Vec<&str> = input.split("\n").collect();
    let mut overlap_count: i32 = 0;
    
    for line in lines.iter() {
        let ranges:Vec<&str> = line.split(",").collect();
        if ranges.len() == 2 && ranges_overlap(ranges[0], ranges[1]) {
            overlap_count += 1;
            // println!("Added one to count: {},{},{}", ranges[0], ranges[1], overlap_count);
        } 
    }    
    
    println!("Overlapping sections: {}", overlap_count);

    overlap_count
}

pub fn ranges_overlap(i1: &str, i2: &str) -> bool {
    let r1:Vec<i32> = i1.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
    let r2:Vec<i32> = i2.split("-").map(|s| s.parse::<i32>().unwrap()).collect();

    (r1[0] <= r2[1] && r1[1] >= r2[0]) || (r2[0] <= r1[1] && r2[1] >= r1[0])
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn ranges_overlap_01(){
        let r1 = "5-7";
        let r2 = "7-9";

        let res = ranges_overlap(r1,r2);

        assert_eq!(res, true)
    }

    #[test]
    fn ranges_overlap_02(){
        let r1 = "2-4";
        let r2 = "6-8";

        let res = ranges_overlap(r1,r2);

        assert_eq!(res, false)
    }

    #[test]
    fn ranges_overlap_03(){
        let r1 = "2-3";
        let r2 = "4-5";

        let res = ranges_overlap(r1,r2);

        assert_eq!(res, false)
    }

    #[test]
    fn ranges_overlap_04(){
        let r1 = "8-96";
        let r2 = "5-6";

        let res = ranges_overlap(r1,r2);

        assert_eq!(res, false)
    }

    #[test]
    fn part_02__works(){
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let res = part_02(input.to_string());

        assert_eq!(res, 4);
    }
}