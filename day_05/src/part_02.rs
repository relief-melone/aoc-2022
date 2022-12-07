use std::time::Instant;
use std::error::Error;
use crate::part_01::get_initial_stacks;
use crate::input_reader::read_file_in_cwd;
use crate::MoveInstruction;
use std::collections::VecDeque;

pub fn run() -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    let file = read_file_in_cwd("assets/input.txt");
    part_02(file);
    println!("Part 2 completed in: {:.2?}", started.elapsed());

    Ok(())
}

pub fn part_02(input:String){
    let lines:Vec<&str> = input.split("\n").collect();
    let mut stack = get_initial_stacks(&lines);
    run_instructions_on_set(&mut stack, &lines);
    
    let mut res:String = " ".to_string();
    for column in stack.iter() {
        res.push(column[column.len()-1].chars().nth(1).unwrap());
    };

    println!("Crates on top are: {}", res);

}

pub fn run_instructions_on_set(stack: &mut VecDeque<VecDeque<String>>, lines: &Vec<&str>){
    for line in lines.iter() {
        if let Some(char) = line.chars().nth(0) {
            if char == 'm' {
                let instruction = MoveInstruction::from_line(line.to_string());
                execute_instruction_9001(stack, instruction);
            }
        }
    }
}

pub fn execute_instruction_9001(stack: &mut VecDeque<VecDeque<String>>, instruction: MoveInstruction){
    let from = instruction.from as usize;
    let to = instruction.to as usize;
    let column = &mut stack[from-1];    
    
    let mut crate_buffer = VecDeque::<String>::new();

    for _i in 0..instruction.count {
        crate_buffer.push_front(column.pop_back().unwrap());        
    }
    for _i in 0..crate_buffer.len() {
        stack[to-1].push_back(crate_buffer.pop_front().unwrap())        
    }
}