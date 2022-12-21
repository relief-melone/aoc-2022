use std::borrow::BorrowMut;
use std::time::Instant;
use std::vec;
use std::{collections::HashMap, hash::Hash};
use std::ops::{Range, RangeInclusive};
use colored::Colorize;
mod input_reader;


#[derive(PartialEq, Debug)]
enum Direction {
    column,
    row
}


#[derive(Debug, PartialEq)]
pub struct Tree {
    height: i32,
    visible: bool
}
impl Tree {
    pub fn new(height: i32) -> Self {
        Tree {
            height,
            visible: false
        }
    }
}

pub struct Grid{
    trees: HashMap<(usize, usize), Tree>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn print(&self){
        for y in 0..=self.max_y {
            let mut line = "".to_string();
            for x in 0..=self.max_x {
                let tree = self.trees.get(&(x,y)).unwrap();
                if tree.visible {
                    line.push_str(
                        format!("{}",tree.height.to_string().as_str().green()).as_str()
                    )
                } else {
                    line.push_str(
                        format!("{}",tree.height.to_string().as_str().red()).as_str()
                    )
                }
            }

            println!("{}", line);
        }
    }

    pub fn trees_visible(&self) -> i32 {
        let mut vis_count:i32 = 0;
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let tree = self.trees.get(&(x,y)).unwrap();
                if tree.visible {
                   vis_count += 1;
                }
            }
        };
        vis_count
    }

    pub fn get_view_distances_product(&self, x: usize, y: usize) -> usize{

        let mut product = 1;

        let incrementors:Vec<( i32, i32)> = vec![
            (0,1),
            (1,0),
            (0,-1),
            (-1,0)
        ];

        for incrementor in incrementors.iter() {
            product *= self.get_view_distance((x,y), incrementor );
        };

        product as usize
    }

    pub fn get_view_distance(&self, start: (usize, usize), incrementor: &(i32, i32)) -> i32{
        let mut current_position = (start.0 as i32 + incrementor.0, start.1 as i32 + incrementor.1);
        let my_tree = self.trees.get(&start).unwrap();
        let mut view_distance = 0;

        while let Some(tree) = self.trees.get(&(current_position.0 as usize, current_position.1 as usize)) {
            view_distance += 1;
            current_position = (current_position.0 + incrementor.0, current_position.1 + incrementor.1);
            if tree.height >= my_tree.height {
                break;
            }
        };

        view_distance
    }
}

pub fn part_01(){
    let input = input_reader::read_file_in_cwd("assets/input.txt");
    let started = Instant::now();
    let mut grid = parse_grid(&input);
    
    calc_visibility(&mut grid);
    println!("Trees visible: {}", grid.trees_visible());
    println!("Execution took {:?}", Instant::now()-started);
    grid.print();
}

pub fn part_02(){
    let input = input_reader::read_file_in_cwd("assets/input.txt");
    let started = Instant::now();

    let grid = parse_grid(&input);
    let mut max_view_distance_score = 0;
    for (coords,_) in &grid.trees {
        let current_viewing_score = grid.get_view_distances_product(coords.0, coords.1);
        if current_viewing_score > max_view_distance_score {
            max_view_distance_score = current_viewing_score;
        }
    }

    println!("Max viewing score was: {}", max_view_distance_score);

    println!("Execution tool {:?}", Instant::now() - started);
}

pub fn parse_grid(input:&str) -> Grid{
    let mut hm: HashMap<(usize,usize), Tree> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    let lines = input.split("\n").collect::<Vec<&str>>();

    for (y, line) in lines.iter().enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            max_x = x;
            hm.insert(
                (x,y), 
                Tree::new(c.to_string().parse::<i32>().unwrap())
            );
        }
    };


    Grid{
        max_x,
        max_y,
        trees: hm,
    }
}

pub fn calc_visibility(grid: &mut Grid){
    let max_x = grid.max_x;
    let max_y = grid.max_y;

    
    let range_combinations = vec![
        ((0..=max_x).collect::<Vec<usize>>(), (0..=max_y).collect::<Vec<usize>>(), Direction::row),
        ((0..=max_x).rev().collect::<Vec<usize>>(), (0..=max_y).collect::<Vec<usize>>(), Direction::row),
        ((0..=max_x).collect::<Vec<usize>>(), (0..=max_y).collect::<Vec<usize>>(), Direction::column),
        ((0..=max_x).collect::<Vec<usize>>(), (0..=max_y).rev().collect::<Vec<usize>>(), Direction::column)
    ];

    
    
    for range_combination in range_combinations {
        let outer_range:Vec<usize>;
        let inner_range:Vec<usize>;

        if range_combination.2 == Direction::row {
            outer_range = range_combination.1;
            inner_range = range_combination.0;
        } else {
            outer_range = range_combination.0;
            inner_range = range_combination.1;
        };

        for i in outer_range.clone() {
            let mut line_of_sight:i32 = -1;

            for j in inner_range.clone() {

                let x:usize;
                let y:usize;

                if range_combination.2 == Direction::row {
                    x = j;
                    y = i;    
                } else {
                    x = i;
                    y = j;
                }

                
                let tree = grid.trees.get_mut(&(x,y)).unwrap();

                if tree.height > line_of_sight {
                    tree.borrow_mut().visible = true;                    
                    line_of_sight = tree.height;
                } else {
                    if line_of_sight == 9 {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_grid_01(){
        let input = input_reader::read_file_in_cwd("assets/test_input_02.txt");
        let grid  = parse_grid(&input);
        let mut expected:HashMap<(usize,usize), Tree> = HashMap::new();
        expected.insert((0,0), Tree::new(1));
        expected.insert((1,0), Tree::new(1));
        expected.insert((2,0), Tree::new(1));
        expected.insert((0,1), Tree::new(1));
        expected.insert((1,1), Tree::new(1));
        expected.insert((2,1), Tree::new(2));
        expected.insert((0,2), Tree::new(0));
        expected.insert((1,2), Tree::new(1));
        expected.insert((2,2), Tree::new(0));

        assert_eq!(grid.trees, expected);

    }

    #[test]
    fn calc_visibility_01(){
        let input = input_reader::read_file_in_cwd("assets/test_input_02.txt");
        let mut grid = parse_grid(&input);
        
        calc_visibility(&mut grid);
        let mut expected:HashMap<(usize,usize), Tree> = HashMap::new();
        expected.insert((0,0), Tree{height: 1, visible: true});
        expected.insert((1,0), Tree{height: 1, visible: true});
        expected.insert((2,0), Tree{height: 1, visible: true});
        expected.insert((0,1), Tree{height: 1, visible: true});
        expected.insert((1,1), Tree{height: 1, visible: false});
        expected.insert((2,1), Tree{height: 2, visible: true});
        expected.insert((0,2), Tree{height: 0, visible: true});
        expected.insert((1,2), Tree{height: 1, visible: true});
        expected.insert((2,2), Tree{height: 0, visible: true});

        println!("Trees visible: {}", grid.trees_visible());

        assert_eq!(grid.trees, expected);
    }
    #[test]
    fn calc_visibility_02(){
        let input = input_reader::read_file_in_cwd("assets/test_input_01.txt");
        let mut grid = parse_grid(&input);
        
        
        calc_visibility(&mut grid);
        grid.print();

        assert_eq!(21, grid.trees_visible())
    }

    #[test]
    fn get_view_distances_product_01(){
        let input = input_reader::read_file_in_cwd("assets/test_input_01.txt");
        let grid = parse_grid(&input);
        
        
        let expected = grid.get_view_distances_product(2, 3);        

        assert_eq!(8 as usize, expected);
    }
}