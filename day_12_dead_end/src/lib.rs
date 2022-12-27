use std::{collections::{HashMap, VecDeque}, borrow::Borrow, time::Instant };

mod input_reader;

#[ derive(Debug, PartialEq, Clone, Default) ]
struct Location {
    visited: bool,
    height: i32,
    point: (usize, usize),
    shortest_path: Vec<(usize,usize)>,
    distance_to_target: Option<usize>
}

#[derive(Debug, PartialEq, Clone)]
struct MapGrid {
    map: HashMap<(usize,usize), Location>,
    starting_point: (usize, usize),
    end_point: (usize, usize),
    to_visit: VecDeque<(usize,usize,usize,usize)>,
    min_steps_start_to_end: usize,
    solutions: HashMap<(usize,usize), usize>
}

pub fn part_01(){
    let started = Instant::now();
    let input = input_reader::read_lines("assets/input.txt");
    let mut mg = MapGrid::new(input);   

    mg.run_until_goal_reached();

    println!("Part 1 - shortest path from S-E takes {} steps", mg.min_steps_start_to_end);
    println!("Execution time for part 1: {:?}", Instant::now() -  started);
}

pub fn part_02(){
    let started = Instant::now();
    let input = input_reader::read_lines("assets/input.txt");
    let mg = MapGrid::new(input.clone());

    let mut shortest_path_overall = 1000;
    for (point, location) in mg.map {
        if location.height == 0 {
            let mut starting_point_in_reach = true;
            println!("Starting run for point: {:?}", point);

            let mut alt_mg = MapGrid::new(input.clone());
            alt_mg.starting_point = point;
            alt_mg.remove_points_out_of_reach(shortest_path_overall);
            
            if let None = alt_mg.map.get(&point) {
                println!("Current starting point out of reach. Won't execute run");
                starting_point_in_reach = false;
            }

            let started_run = Instant::now();
            
            if starting_point_in_reach {
                if let Some(shortest_path_current) = alt_mg.run_until_goal_reached_limited(shortest_path_overall) {
                    if shortest_path_current < shortest_path_overall {
                        shortest_path_overall = shortest_path_current
                    }
                        
                }
            }
            
            println!("Run completed after {:?}. Shortest path currently: {}", Instant::now()-started_run, shortest_path_overall);

        }
    }

    println!("Part 2 - shortest path from a-E takes {} steps", shortest_path_overall);
    println!("Execution time for part 1: {:?}", Instant::now() -  started);
}

impl MapGrid {
    fn new(lines:Vec<String>) -> Self {
        let mut map: HashMap<(usize,usize), Location> = HashMap::new();
        let mut starting_point:Option<(usize, usize)> = None;
        let mut end_point:Option<(usize,usize)> = None;
        
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {

                let location = Location { 
                    visited: false, 
                    height: char_to_elevation(char), 
                    shortest_path: vec![],
                    point: ( x,y ),
                    distance_to_target: None
                };                

                if char == 'S'  { starting_point = Some((x,y)) }
                if char == 'E' { end_point = Some((x,y)) }
                
                map.insert((x,y), location);
            }
        };        

        let start = starting_point.unwrap();

        Self { 
            map,
            to_visit: VecDeque::from( vec![(start.0,start.1,start.0,start.1)] ),
            starting_point: starting_point.unwrap(),
            end_point: end_point.unwrap(),
            min_steps_start_to_end: 0,
            solutions: HashMap::new()
         }
    }

    pub fn solve_path(&mut self){
        let mut steps_from_target = 0;
        for point in self.clone().map.get(&self.end_point).unwrap().shortest_path.clone() {
            steps_from_target += 1;
            let mut location = self.map.get_mut(&point).unwrap();
            location.distance_to_target = Some(steps_from_target);
            
            self.solutions.insert(point, steps_from_target);
        }
    }

    pub fn process_step(&mut self, current_point: (usize,usize, usize, usize), max_steps: usize ) {       

        let map = self.map.clone();

        let current_location = self.map.get_mut(&(current_point.0, current_point.1)).unwrap();
        let previous_point = (current_point.2, current_point.3);

        let previous_location = map.get(&previous_point).unwrap();

        current_location.shortest_path = Vec::from(
            previous_location.shortest_path.borrow(),
        );        

        if current_location.point == self.end_point && self.min_steps_start_to_end == 0{
            self.min_steps_start_to_end = current_location.shortest_path.len();            
        }

        current_location.visited = true;
        current_location.shortest_path.push(current_location.point);


        let new_points_to_visit = get_possible_locations(
            map,
            current_location.point, 
            self.end_point, 
            max_steps - current_location.shortest_path.len() 
        );
        
        for new_point in new_points_to_visit {
            let point_to_visit = (new_point.0,new_point.1, current_point.0, current_point.1);
            if !self.to_visit.contains(&point_to_visit) {
                self.to_visit.push_back(point_to_visit);
            }    
        }

    }

    pub fn remove_points_out_of_reach(&mut self, max_steps: usize){
        for (point, _) in self.map.clone().iter() {
            if !point_is_in_reach(*point, self.end_point, max_steps) {
                self.map.remove(point);
            }
        }
    }

    pub fn run_until_goal_reached_limited(&mut self, max_path_length: usize) -> Option<usize>{

        while let Some(current_point) = self.to_visit.pop_front() {
            self.process_step(current_point, max_path_length);
            if (current_point.0, current_point.1) == self.end_point {
                return Some(self.min_steps_start_to_end)
            }
        };

        None
    }

    pub fn run_until_goal_reached(&mut self){
        while let Some(current_point) = self.to_visit.pop_front() {
            self.process_step(current_point, 5000);
            
            if (current_point.0, current_point.1) == self.end_point {
                break;
            }
        }

        self.solve_path();
    }
   
}


fn char_to_elevation(c:char)->i32{
    if c == 'S'{ 0 }   
    else if c == 'E'{ 25 } 
    else { c as i32-97 }
}

fn point_is_in_reach(current_p: (usize, usize), target_p: (usize, usize), max_steps: usize) -> bool {
    current_p.0.abs_diff(target_p.0) + current_p.1.abs_diff(target_p.1) < max_steps
}

fn get_possible_locations(map: HashMap<(usize,usize), Location>, current_point:(usize,usize), end_point: (usize, usize), max_steps_left: usize) -> Vec<(usize,usize)>{
    let mut possible_locations:Vec<(usize,usize)> = vec![];
        let current_location = map.get(&current_point).unwrap();

        let incrementors:Vec<(i32,i32)> = vec![(0,1),(1,0),(-1,0),(0,-1)];        
        for inc in incrementors {
            let pt = (current_point.0 as i32 - inc.0 as i32, current_point.1 as i32-inc.1 as i32);
            if pt.0 >= 0 && pt.1 >= 0 {

                if let Some(pl) = map.get(&(pt.0 as usize, pt.1 as usize)) {
                    if !point_is_in_reach(current_point, end_point, max_steps_left){
                        // println!("Point {:?} not in reach. Continuing...", current_point);
                        continue;
                    }

                    if !pl.visited && (pl.height - current_location.height) <= 1 {
                        possible_locations.push((pt.0 as usize, pt.1 as usize));
                    }
                }
            }
        };

        possible_locations
}

#[cfg(test)]

mod test {
    use super::*;


    #[test]
    fn map_grid_01(){
        let input:Vec<String> = vec![
        "Sab",
        "fca",
        "aEa"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let mg = MapGrid::new(input);

        assert_eq!(
            mg.map.get(&(0,0)).unwrap(), &Location{ height: 0, shortest_path: vec![], visited: false, point: (0,0), distance_to_target: None}
        );
        assert_eq!(
            mg.map.get(&(1,1)).unwrap(), &Location{ height: 2, shortest_path: vec![], visited: false, point: (1,1), distance_to_target: None }
        );

        assert_eq!(mg.starting_point, (0,0));
        assert_eq!(mg.end_point, (1,2));
        
    }

    #[test]
    fn map_run_01(){
        let input = input_reader::read_lines("assets/input_test.txt");
        let mut mg = MapGrid::new(input);

        mg.run_until_goal_reached();

        println!("{:#?}", mg);

        assert_eq!(mg.min_steps_start_to_end, 31);
    }
}