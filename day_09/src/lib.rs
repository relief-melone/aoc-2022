mod input_reader;
use std::{collections::HashMap, time::Instant };

pub fn part_01(){
    let started = Instant::now();
    let lines = input_reader::read_lines("assets/input.txt");
    let mut rope = Rope::new(1);

    for line in lines {
        let movement = line_to_movement(&line);
        rope.move_head(movement);
    }

    println!("Part 1 - Positions visited by tail: {}", rope.positions_visited_by_tail.len());
    println!("Execution time: {:?}", Instant::now() - started);
}

pub fn part_02(){
    let started = Instant::now();
    let lines = input_reader::read_lines("assets/input.txt");
    let mut rope = Rope::new(9);

    for line in lines {
        let movement = line_to_movement(&line);
        rope.move_head(movement);
    }

    println!("Part 2 - Positions visited by tail: {}", rope.positions_visited_by_tail.len());
    println!("Execution time: {:?}", Instant::now() - started);
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    U,
    D,
    R,
    L
}

#[derive(Clone, Debug, PartialEq)]
struct Rope {
    head: (i32, i32),
    knots: Vec<(i32, i32)>,
    positions_visited_by_tail: HashMap<(i32, i32), bool>
}


impl Rope {
    pub fn new(tail_length:i32) -> Self {
        let mut s = Self {
            head: (0,0),
            knots: vec![],
            positions_visited_by_tail: HashMap::new()
        };

        for _ in 0..tail_length {
            s.knots.push((0,0))
        }

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

            self.move_knots();
        }

    }

    pub fn move_knots(&mut self) -> (){
        let knot_len = &self.knots.len();
        let mut parent = self.head;
        for (index, knot) in self.knots.iter_mut().enumerate() {
            *knot = move_knot(&knot, &parent);
            parent = *knot;

            if index == *knot_len-1 {
                self.positions_visited_by_tail.insert(*(knot), true);
            }
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

pub fn knot_detached(current_knot: &(i32,i32), previous_knot:&(i32,i32)) -> bool {
    (previous_knot.0 - current_knot.0).abs() > 1 || (previous_knot.1 - current_knot.1).abs() > 1
}

pub fn prev_is_horizontal_or_vertical(current_knot: &(i32,i32), previous_knot:&(i32,i32))-> bool{
    previous_knot.0 - current_knot.0 == 0 || previous_knot.1 - current_knot.1 == 0
}

pub fn move_knot(current_knot: &(i32,i32), previous_knot: &(i32,i32)) -> (i32,i32){
   
    let dx = previous_knot.0.cmp(&current_knot.0);
    let dy= previous_knot.1.cmp(&current_knot.1);
    let inc = if knot_detached(current_knot, previous_knot) {        
        (dx as i32, dy as i32)
    } else {
        (0,0)
    };

    (current_knot.0 + inc.0, current_knot.1 + inc.1) 

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
        let mut rope = Rope::new(1);

        for line in lines {
            let movement = line_to_movement(&line);
            rope.move_head(movement);
        }

        assert_eq!(rope.positions_visited_by_tail.len(), 13);
        
    }
    
}