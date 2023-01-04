use std::{hash::Hash, collections::HashMap, ops::RangeInclusive, time::Instant };

use input_reader::read_lines;

mod input_reader;

#[derive(Debug, PartialEq, Hash, Clone)]
struct Sensor {
    pos: Coord,
    range: i32,
    beacon: Coord
}

impl Sensor {
    fn new(line: &str)-> Self {
        let (sc, bc) = get_coords_from_line(line);
        Self { 
            pos: sc, 
            range: get_manhatten_distance(&sc, &bc), 
            beacon: bc 
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Coord (i32, i32);
impl Coord {
    fn x(&self) -> i32 { self.0 }
    fn y(&self) -> i32 { self.1 }
}

#[derive(Clone, Debug, PartialEq)]
struct MultiRange(Vec<RangeInclusive<i32>>);
impl MultiRange {
    fn new() -> Self {
        Self(vec![])
    }

    fn add_range(&mut self, from:i32, to:i32){
        
        let f = if from < to { from } else { to };
        let t = if to > from { to } else { from };

        self.0.push(f..=t);
        self.compact();

    }

    fn compact(&mut self){
        let mut merged_ranges:Vec<RangeInclusive<i32>> = vec![];
        
        let mut ranges = self.0.clone();
        ranges.sort_by(|a,b|{
            a.start().cmp(b.end())
        });

        let mut r_it = ranges.iter().peekable();

        while let Some(range) = r_it.next(){
            if let Some(last_item) = merged_ranges.last_mut() {
                if ranges_overlap_or_adjacent(range, &last_item){
                    *last_item = combine_ranges(
                        &last_item, &range.clone()
                    );
                    continue;                
                }
            } 

            if let Some(&next_range) = r_it.peek() {
                if ranges_overlap_or_adjacent(range, next_range) {
                    merged_ranges.push(combine_ranges(range, next_range));
                    r_it.next();
                }
                else {
                    merged_ranges.push(range.clone());
                }
            } else {
                merged_ranges.push(range.clone());
            }
        }

        merged_ranges.sort_by(|a,b| {
            a.start().cmp(b.start())
        });

        self.0 = merged_ranges;        
    }

    fn contains(&self, x: i32) -> bool {        
        for r in self.clone().0 {
            if r.contains(&x) {
                return true;
            }
        }
        false
    }

    fn not_in_range(&self, cmp: RangeInclusive<i32>) -> Vec<i32> {
        let mut n_ir:Vec<i32> = vec![];
        let mut i = *cmp.start();
        let mut iter_ranges = self.0.clone().into_iter();
        while i <= *cmp.end() {
            if let Some(r) = &iter_ranges.next() {
                while *r.start() > i {
                    n_ir.push(i);
                    i += 1;
                }
                i = r.end()+1;                
            } else {
                break;
            }
        }
        n_ir

    }

    fn count(&self) -> i32 {
        let mut count = 0;
        println!("init count: {}", count);

        for range in &self.0 {
            count += range.end() - range.start() + 1;            
            println!("count: {}", count);
        }

        count
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Grid {
    sensors: HashMap<Coord, Sensor>,
    dimensions: (Coord, Coord)
        
}
impl Grid {
    fn new(input: Vec<String>) -> Self {

        let mut grid = Self{
            sensors: HashMap::new(),
            dimensions: (Coord(0,0), Coord(0,0))
        };

        for line in input.iter() {
            let s = Sensor::new(line);
            let s_clone = s.clone();
            
            grid.add_sensor(s);            

            Grid::update_dimensions_with_sensor(&mut grid, &s_clone);
        }

        grid
    }

    fn add_sensor(&mut self, s: Sensor){
        self.sensors.insert(s.pos, s);
    }

    fn update_dimensions_with_sensor(&mut self, s: &Sensor){        

        self.dimensions = (
            Coord(self.dimensions.0.x().min(s.pos.x() - s.range), self.dimensions.0.y().min(s.pos.y() - s.range)),
            Coord(self.dimensions.1.x().max(s.pos.y() + s.range), self.dimensions.1.y().max(s.pos.y() + s.range))
        )
    }

    fn get_mr_for_y(&self, y: i32) -> MultiRange {
        let mut mr = MultiRange::new();        
        
        for (_, sensor) in self.sensors.clone() {           
            if let Some(range) = get_known_empty_range_for_line_and_sensor(&sensor, y){
                mr.add_range(*range.start(), *range.end());
            }        
        }

        mr
    }


    fn find_beacon(&self, xr: &RangeInclusive<i32>, yr: &RangeInclusive<i32>) -> Option<Coord>{
        for y in yr.clone() {
            // println!("Processing y: {}", y);
            let mr = self.get_mr_for_y(y);
            let n_ir = mr.not_in_range(xr.clone());
            if n_ir.len() > 0 {
                return Some(Coord(n_ir[0], y));
            }

        }
        None
    }


    fn get_empty_count_for_y(&self, y: i32) -> i32{
        let mr = self.get_mr_for_y(y);
        let mut sensor_positions_present: Vec<i32> = vec![];

        for sensor in self.sensors.clone().values(){
            if sensor.pos.y() == y && mr.contains(sensor.pos.x() ){
                sensor_positions_present.push(sensor.pos.x())
            }

            if sensor.beacon.y() == y && mr.contains(sensor.beacon.x() ){ 
                sensor_positions_present.push(sensor.beacon.x())
            }
        }
        sensor_positions_present.sort();        
        sensor_positions_present.dedup();
        let count = mr.count();
        count - sensor_positions_present.len() as i32
    }
    
}

pub fn part_01(path: Option<&str>) {
    let started = Instant::now();
    let path = path.unwrap_or("assets/input.txt");
    let input = read_lines(path);
    let grid = Grid::new(input);

    println!("Part 01 - {} positions cannot contain a beacon", grid.get_empty_count_for_y(2000000));
    println!("Part 01 - Execution time {:?}", Instant::now()- started)
}

pub fn part_02(path: Option<&str>) {
    let started = Instant::now();
    let path = path.unwrap_or("assets/input.txt");
    let input = read_lines(path);
    let grid = Grid::new(input);
    let beacon = grid.find_beacon(&(0..=4000000), &(0..=4000000));
    
    if let Some(beacon) = beacon {
        
        let frequency:i128 = ((beacon.x() as i128)*4000000 + beacon.y() as i128) as i128;
        println!("Part 02 - Coords of beacon are x: {}, y: {} frequency is: {}", beacon.x(), beacon.y(), frequency );
        println!("Part 02 - Execution time {:?}", Instant::now()- started)
    }    
}


fn get_known_empty_range_for_line_and_sensor(sensor: &Sensor, y: i32) -> Option<RangeInclusive<i32>> {
    let distance_left = sensor.range - (y - sensor.pos.1).abs();    
    if distance_left < 0 {
        None
    } else {
        Some((sensor.pos.0 - distance_left)..=(sensor.pos.0 + distance_left ))
    }
}

fn sort_ranges<'a>( r1: &'a RangeInclusive<i32>, r2: &'a RangeInclusive<i32>) -> (RangeInclusive<i32>, RangeInclusive<i32>)
{
    let r1_min = r1.start();
    let r2_min = r2.end();
    if r1_min < r2_min { (r1.clone(), r2.clone()) } else { (r2.clone(), r1.clone()) }
}

fn combine_ranges(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    if !ranges_overlap_or_adjacent(r1, r2) { panic!("Cannot combine ranges that don't overlap") }

    let min = if r1.start() < r2.start() { *r1.start() } else { *r2.start() };
    let max = if r1.end() > r2.end() { *r1.end() } else { *r2.end() };

   min..=max
}

fn ranges_overlap_or_adjacent(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    let (r1, r2) = sort_ranges(r1, r2);
    *r1.end() >= *r2.start() - 1
}

fn get_coords_from_line(line:&str) -> (Coord, Coord){

    let mut numbers:Vec<i32> = vec![];

    for item in line.split([',',':','=']) {
        if let Ok(num) = item.parse::<i32>() {
            numbers.push(num);
        }
    };

    if numbers.len() != 4 {
        panic!("Could not find 4 numbers in line but {}", numbers.len());
    }

    (Coord(numbers[0], numbers[1]), Coord(numbers[2], numbers[3]))
}

fn get_manhatten_distance(p1: &Coord, p2: &Coord) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}


#[cfg(test)]
mod test {
    use crate::input_reader::read_lines;

    use super::*;

    #[test]
    fn get_coords_from_line_01(){
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let output = get_coords_from_line(input);

        let s_exp = Coord(2,18);
        let b_exp = Coord(-2,15);

        assert_eq!(output, (s_exp, b_exp));


    }

    #[test]
    fn get_manhattan_distance_01(){
        let from = Coord(1,-5);
        let to = Coord(-5,5);

        let distance = get_manhatten_distance(&to, &from);
        assert_eq!(distance, 16);
    }

    #[test]
    fn multi_range_01(){
        let mut mr = MultiRange::new();
        mr.add_range(0,5);
        mr.add_range(3, 6);

        assert_eq!(mr.0.len(), 1);
        assert_eq!(mr.0[0], 0..=6);
    }

    #[test]
    fn multi_range_02(){
        let mut mr = MultiRange::new();
        mr.add_range(0,5);
        mr.add_range(7, 12);

        assert_eq!(mr.0.len(), 2);
        assert_eq!(mr.0[0], 0..=5);
        assert_eq!(mr.0[1], 7..=12);
    }

    #[test]
    fn multi_range_03(){
        let mut mr = MultiRange::new();
        mr.add_range(12,6);
        mr.add_range(2, 5);

        assert_eq!(mr.0.len(), 1);
        assert_eq!(mr.0[0], 2..=12);
    }

    #[test]
    fn multi_range_04(){
        let mut mr = MultiRange::new();
        mr.add_range(1,5);
        mr.add_range(7, 10);
        mr.add_range(4,8);

        assert_eq!(mr.0.len(), 1);
        assert_eq!(mr.0[0], 1..=10);
    }

    #[test]
    fn grid_01(){
        let input = read_lines("assets/input_test_01.txt");
        let grid = Grid::new(input);
        let res = grid.get_empty_count_for_y(10);

        assert_eq!(res, 26);
    }

    #[test]
    fn grid_02(){
        let input = read_lines("assets/input_test_01.txt");
        let grid = Grid::new(input);

        let res = grid.get_empty_count_for_y(9);

        assert_eq!(res, 25);
    }

    #[test]
    fn grid_03(){
        let input = read_lines("assets/input_test_01.txt");
        let grid = Grid::new(input);

        let res = grid.get_empty_count_for_y(11);

        assert_eq!(res, 27);
    }

    #[test]
    fn ranges_overlap_01(){
        let r1 = 0..=5;
        let r2 = 5..=6;

        assert!(ranges_overlap_or_adjacent(&r1, &r2));
    }

    #[test]
    fn ranges_overlap_02(){
        let r2 = 0..=4;
        let r1 = 5..=6;

        assert!(ranges_overlap_or_adjacent(&r1, &r2));
    }

    #[test]
    fn ranges_overlap_03(){
        let r1 = -2..=1;
        let r2 = -1..=5;

        assert!(ranges_overlap_or_adjacent(&r1, &r2));
    }

    #[test]
    fn find_empty_coord_01(){
        let input = read_lines("assets/input_test_01.txt");
        let grid = Grid::new(input);

        let res = grid.find_beacon(&(0..=20), &(0..=20));

        assert_eq!(res, Some(Coord(14,11)));
    }

    #[test]
    fn compact_01(){
        let mut mr = MultiRange(
            vec![
                0..=5,
                7..=10,
                6..=8
            ]
        );
        mr.compact();
        

        assert_eq!(mr.0.len(), 1);
    }

    #[test]
    fn compact_02(){
        let mut mr = MultiRange(
            vec![
                -2..=14,
                12..=12,
                15..=24,
                14..=18
            ]
        );
        mr.compact();
        

        assert_eq!(mr.0.len(), 1);
    }
}