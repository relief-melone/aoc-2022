mod input_reader;
use std::{cmp::{self, Ordering}, ops::Not, collections::HashMap, time::Instant};

pub fn part_01(){
    let started = Instant::now();
    let lines = input_reader::read_lines("assets/input.txt");
    let mut rope = Rope::new();

    for line in lines {
        let movement = line_to_movement(&line);
        rope.move_head(movement);
    }

    println!("Positions visited by tail: {}", rope.positions_visited_by_tail.len());
    println!("Execution time: {:?}", Instant::now() - started);
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    U,
    D,
    R,
    L
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    positions_visited_by_tail: HashMap<(i32, i32), bool>
}

impl Rope {
    pub fn new() -> Self {
        let mut s = Self {
            head: (0,0),
            tail: (0,0),
            positions_visited_by_tail: HashMap::new()
        };

        s.positions_visited_by_tail.insert((0,0), true);
        s

    }

    pub fn move_head(&mut self, steps: (Direction, i32)) -> (){
        for _ in 0..steps.1 {
            match steps.0 {
                Direction::L => self.head.0 -= 1,
                Direction::R => self.head.0 += 1,
                Direction::D => self.head.1 -= 1,
                Direction::U => self.head.1 += 1
            }
            self.move_tail(steps.0);
        }

    }

    pub fn tail_and_head_detached(&self) -> bool {
        (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1
    }

    pub fn move_tail(&mut self, head_moved: Direction) -> (){
        
        let incrementor =
            match (head_moved, self.head.0.cmp(&self.tail.0), self.head.1.cmp(&self.tail.1)) {
            (Direction::R, Ordering::Greater, Ordering::Equal) => (1,0),
            (Direction::R, Ordering::Greater, Ordering::Greater) => (1,1),
            (Direction::R, Ordering::Greater, Ordering::Less) => (1,-1),
            (Direction::L, Ordering::Less, Ordering::Equal) => (-1,0),
            (Direction::L, Ordering::Less, Ordering::Greater) => (-1,1),
            (Direction::L, Ordering::Less, Ordering::Less) => (-1,-1),
            (Direction::U, Ordering::Equal, Ordering::Greater) => (0,1),
            (Direction::U, Ordering::Greater, Ordering::Greater) => (1,1),
            (Direction::U, Ordering::Less, Ordering::Greater) => (-1,1),
            (Direction::D, Ordering::Equal, Ordering::Less) => (0,-1),
            (Direction::D, Ordering::Greater, Ordering::Less) => (1,-1),
            (Direction::D, Ordering::Less, Ordering::Less) => (-1,-1),
            _ => (0,0)
        };

        if self.tail_and_head_detached() {
            self.tail.0 += incrementor.0;
            self.tail.1 += incrementor.1;
            self.positions_visited_by_tail.insert((self.tail.0, self.tail.1), true);
        }
    }
}

pub fn line_to_movement(line:&str)-> (Direction, i32) {
    let chars = line.split(" ").collect::<Vec<&str>>();
    let steps = chars[1].to_string().parse::<i32>().unwrap();

    match chars[0] {
        "R" => (Direction::R, steps),
        "L" => (Direction::L, steps),
        "U" => (Direction::U, steps),
        "D" => (Direction::D, steps),
        _ => panic!("{} is not a known direction", chars[0])
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_to_movement_01(){
        let commands = vec![
            "R 2",
            "U 39",
            "D 1",
            "L 6"
        ];
        let correct_movements = vec![
            (Direction::R,2),
            (Direction::U,39),
            (Direction::D,1),
            (Direction::L,6)
        ];

        for (index, command) in commands.iter().enumerate() {
            assert_eq!(correct_movements[index], line_to_movement(command));
        }
    }

    #[test]
    fn rope_01(){
        let lines = input_reader::read_lines("assets/input_test_01.txt");
        let mut rope = Rope::new();

        for line in lines {
            let movement = line_to_movement(&line);
            rope.move_head(movement);
        }

        assert_eq!(rope.positions_visited_by_tail.len(), 13);
        
    }
    
}