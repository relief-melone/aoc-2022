use std::{collections::HashMap, time::Instant };

use input_reader::read_lines;
mod input_reader;

#[derive(Debug, Clone)]
enum FieldType {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: HashMap<(usize, usize), FieldType>,
    moving_particles: Vec<(usize, usize)>,
    sand_source: (usize, usize),    
    min_x: usize,
    max_x: usize,
    max_y: usize,
    has_floor: bool
}

pub fn part_01(path: Option<&str>) -> i32 {
    let started = Instant::now();
    let input = read_lines(path.unwrap_or("assets/input.txt"));
    let mut grid = Grid::new(input,false);
    let mut resting_grains_count = 0;
    let mut grain_comes_to_rest: Result<(),()> = Ok(());

    while let Ok(()) = grain_comes_to_rest {
        grain_comes_to_rest = grid.run_single_sand_grain();
        if let Ok(()) = grain_comes_to_rest { resting_grains_count += 1; }   
    }

    println!("Part 01 - Grains that can be processed: {}", resting_grains_count);
    println!("Execution time for part 1: {:?}", Instant::now() - started);    

    resting_grains_count
}

pub fn part_02(path: Option<&str>) -> i32 {
    let started = Instant::now();
    let input = read_lines(path.unwrap_or("assets/input.txt"));
    let mut grid = Grid::new(input, true);
    let mut resting_grains_count = 0;
    let mut grain_comes_to_rest: Result<(),()> = Ok(());

    while let Ok(()) = grain_comes_to_rest {
        grain_comes_to_rest = grid.run_single_sand_grain();
        resting_grains_count += 1; 
    }

    println!("Part 02 - Grains that can be processed: {}", resting_grains_count);
    println!("Execution time for part 2: {:?}", Instant::now() - started);    
    // grid.draw();
    resting_grains_count
}

impl Grid {
    fn new(input: Vec<String>, has_floor:bool)-> Self {

        let mut grid:HashMap<(usize,usize), FieldType> = HashMap::new();
        let mut max_x = 0;
        let mut min_x = usize::MAX;
        let mut max_y = 0;

        for line in input {
            let mut instructions = 
            line.as_str()
            .split(" -> ")
            .map(|i| { i.trim() })
            .peekable();            

            while let (
                Some(current_instruction),
                Some(&next_instruction)
            ) = (instructions.next(), instructions.peek()) {
                let pt1 = current_instruction.split(",").map(|pos|{pos.parse::<usize>().unwrap()}).collect::<Vec<usize>>();
                let pt2 = next_instruction.split(",").map(|pos|{pos.parse::<usize>().unwrap()}).collect::<Vec<usize>>();

                let range1 = if pt1[0] > pt2[0] { pt2[0]..=pt1[0] } else { pt1[0]..=pt2[0] };
                let range2 = if pt1[1] > pt2[1] { pt2[1]..=pt1[1] } else { pt1[1]..=pt2[1] };


                for x in range1.clone() {
                    if x < min_x { min_x = x };
                    if x > max_x { max_x = x };                    

                    for y in range2.clone() {
                        if y > max_y { max_y = y };
                        grid.insert((x,y), FieldType::Rock);
                    }
                };

            }
            
        };        

        Self {
            grid,
            moving_particles: Vec::new(),
            sand_source: (500, 0),
            max_x,
            min_x,
            max_y,
            has_floor
        }
    }

    #[allow(dead_code)]
    fn draw(&self){
        for y in 0..=(self.max_y+3) {
            let mut line = "".to_string();
            for x in self.min_x..=self.max_x {
                match self.grid.get(&(x,y)) {
                    Some(FieldType::Rock) => line.push('#'),
                    Some(FieldType::Sand) => line.push('o'),                    
                    _ => line.push(' ')
                }
            }
            println!("{}", line);
        }
    }

    #[allow(dead_code, unused)]
    fn tick(&mut self) -> Result<usize, &str> {
        let mut indices_not_moving_anymore:Vec<usize> = vec![];        
        let mut max_x = self.max_x;
        let mut min_x = self.min_x;

        for (index, particle) in self.moving_particles.iter_mut().enumerate() {
            let old_position = particle.clone();
            
            *particle = match (
                self.grid.get(&(particle.0, particle.1 + 1)),
                self.grid.get(&(particle.0-1, particle.1 + 1)),
                self.grid.get(&(particle.0+1, particle.1 + 1)),
            ) {
                (None, _, _) => (particle.0, particle.1 + 1),
                (Some(_), None, _) => (particle.0 -1, particle.1 + 1),
                (Some(_), Some(_), None) => (particle.0 +1, particle.1 + 1 ),
                _ => *particle
            };

            if old_position == *particle || (self.has_floor && particle.1 == self.max_y+1) {
                self.grid.insert(*particle, FieldType::Sand);
                indices_not_moving_anymore.push(index);                
            }            
            
            if !self.has_floor && particle.1 > self.max_y {
                return Err("particle will not come to rest anymore");
            }

            if self.has_floor && *particle == (self.sand_source){
                return Err("particle is already at sand source");
            }
        }

        for ind in indices_not_moving_anymore {
            self.moving_particles.remove(ind);
        };   

        Ok(self.moving_particles.len())

    }

    fn run_single_sand_grain(&mut self) -> Result<(),()>{
        self.moving_particles.push(self.sand_source.clone());
        
        while self.moving_particles.len() > 0 {
            if let Err(_) = self.tick() {
                return Err(());
            }            
        }

        Ok(())
    }
}


#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn part01_01(){
        let grains_processed = part_01(Some("assets/input_test_01.txt"));

        assert_eq!(grains_processed, 24)        
    }

    #[test]
    fn part02_01(){
        let grains_processed = part_02(Some("assets/input_test_01.txt"));

        assert_eq!(grains_processed, 93)
    }
}