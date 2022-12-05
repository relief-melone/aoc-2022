use std::error::Error;
pub mod helpers;
use helpers::Config;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(unused_must_use)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

struct Hand {
    shape: Shape,
    beats_shape: Shape
}
impl Hand {   
    pub fn new(shape: Shape) -> Self {
        match shape {
            Shape::Rock => Self { shape: Shape::Rock, beats_shape: Shape::Scissors },
            Shape::Paper => Self { shape: Shape::Paper, beats_shape: Shape::Rock },
            Shape::Scissors => Self { shape: Shape::Scissors, beats_shape: Shape::Paper }
        }
    }

    pub fn play_against(self, enemies_hand: &Hand) -> i32{
        let mut sum:i32 = 0;
        match self.shape {
            Shape::Rock => sum = sum + 1,
            Shape::Paper => sum = sum + 2,
            Shape::Scissors => sum = sum + 3
        };

        if self.shape == enemies_hand.shape {
            sum = sum + 3;
        }
        if self.beats_shape == enemies_hand.shape {
            sum = sum + 6;
        }

        sum
    }
    
}

struct Round {
    enemies_hand: Hand,
    own_hand: Hand
}

impl Round {
    pub fn new(line:String, config: &Config) -> Self {
        if config.part == 1 {
            return Round::new_01(line);
        } else {
            return Round::new_02(line);
        }
    }

    pub fn new_01(line: String) -> Self{
        let input:Vec<&str> = line.split(" ").collect();        

        let enemies_hand = match input[0]{
            "A" => Hand::new(Shape::Rock),
            "B" => Hand::new(Shape::Paper),
            "C" => Hand::new(Shape::Scissors),
            _ => panic!("unkown input for enemies hand")
        };

        let own_hand = match input[1]{
            "X" => Hand::new(Shape::Rock),
            "Y" => Hand::new(Shape::Paper),
            "Z" => Hand::new(Shape::Scissors),
            _ => panic!("unknown input for own hand")
        };

        Self { enemies_hand, own_hand }
    }

    pub fn new_02(line: String) -> Self {
        let input:Vec<&str> = line.split(" ").collect();        

        let enemies_hand = match input[0]{
            "A" => Hand::new(Shape::Rock),
            "B" => Hand::new(Shape::Paper),
            "C" => Hand::new(Shape::Scissors),
            _ => panic!("unkown input for enemies hand")
        };

        let own_hand = match input[1]{
            "X" => Hand::new(enemies_hand.beats_shape),
            "Y" => Hand::new(enemies_hand.shape),
            "Z" => Hand::new(Hand::new(enemies_hand.beats_shape).beats_shape),
            _ => panic!("unknown input for own hand")
        };

        Self { enemies_hand, own_hand }
    }



    pub fn get_rounds_points(self) -> i32 {
        self.own_hand.play_against(&self.enemies_hand)
    }
}


pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    if config.part == 1{
        run_part_01();
    } else {
        run_part_02();
    }

    Ok(())
}

pub fn run_part_01() -> Result<(), Box<dyn Error>>{

    let lines = helpers::read_lines("assets/input.txt").unwrap();
    let mut points:i32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let round = Round::new_01(line);
            points = points + round.get_rounds_points();
        }
    };

    println!("Total points: {}", points);
    

    Ok(())
}

pub fn run_part_02() -> Result<(), Box<dyn Error>>{
    let lines = helpers::read_lines("assets/input.txt").unwrap();
    let mut points:i32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let round = Round::new_02(line);
            points = points + round.get_rounds_points();
        }
    };

    println!("Total points: {}", points);
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_winning_points__rock_wins(){
        let my_hand = Hand::new(Shape::Rock);
        let enemies_hand = Hand::new(Shape::Scissors);

        let points = my_hand.play_against(&enemies_hand);

        assert_eq!(points, 7)
    }

    #[test]
    fn get_winning_points__draw(){
        let my_hand = Hand::new(Shape::Paper);
        let enemies_hand = Hand::new(Shape::Paper);

        let points = my_hand.play_against(&enemies_hand);

        assert_eq!(points, )
    }
}

