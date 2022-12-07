use std::time::Instant;
use std::error::Error;
use crate::input_reader::read_file_in_cwd;
use std::collections::VecDeque;
use crate::MoveInstruction;


pub fn run() -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    let file = read_file_in_cwd("assets/input.txt");
    part_01(file);
    println!("Part 1 completed in: {:.2?}", started.elapsed());

    Ok(())
}

pub fn part_01(input:String){
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
                execute_instruction(stack, instruction);
            }
        }
    }
}

pub fn get_initial_stacks(lines: &Vec<&str>) -> VecDeque<VecDeque<String>>{
    let mut stack:VecDeque<VecDeque<String>> = VecDeque::new();

    for (index, line) in lines.iter().enumerate() {        
        match line.chars().nth(0) {
            Some(char) => if char == 'm' { continue },
            None => continue
        }

        let level = line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        for (i_cs, crate_spot) in level.iter().enumerate() {

            if index == 0 {
                stack.push_back(VecDeque::new());
            }

            if crate_spot.chars().nth(0).unwrap() != ' ' {
                stack[i_cs].push_front(crate_spot.trim().to_string());
            }
        }        
    }
    stack
}

pub fn execute_instruction(stack: &mut VecDeque<VecDeque<String>>, instruction: MoveInstruction){
    for _i in 0..instruction.count {
        let from = instruction.from as usize;
        let to = instruction.to as usize;
        let crate_to_move = stack[from-1].pop_back().unwrap();
        stack[to-1].push_back(crate_to_move)
    }
}



#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn get_initial_stacks__01(){
        let input = vec!(
            "[D]        ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 "
        );
        let output = get_initial_stacks(&input);
        
        assert_eq!(output, vec!(
            vec!("[Z]","[N]","[D]"),
            vec!("[M]","[C]"),
            vec!("[P]")
        ))

    }

    #[test]
    fn MoveInstruction_from_line_01() {
        let input = "move 2 from 7 to 2";
        let mi = MoveInstruction::from_line(input.to_string());

        assert_eq!(
            mi,
            MoveInstruction{count:2,from:7,to:2}
        )
    }

    #[test]
    fn execute_instruction_01(){
        let instruction = MoveInstruction::from_line("move 1 from 1 to 2".to_string());
        let mut original_stack = get_initial_stacks(&vec!(
            "[D]        ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 "
        ));
        let expected_output_stack = get_initial_stacks(&vec!(
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 "
        ));

        execute_instruction(&mut original_stack, instruction);
        assert_eq!(expected_output_stack, original_stack);
    }


}