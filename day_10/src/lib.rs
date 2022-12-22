use std::{collections::VecDeque, time::Instant};

mod input_reader;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Noop,
    Addx(i32)
}
pub struct CPU {
    instructions: VecDeque<Instruction>,
    cycle: i32,
    register: i32,
    current_instruction: Option<Instruction>,
    ticks_to_completion: i32,
    sum_of_signal_strenghts: i32,
}

pub fn part_01(){
    let started = Instant::now();
    let mut cpu = CPU::new("assets/input.txt");
    cpu.run();
    println!("Part 1 - Sum of signal strengths: {}", cpu.sum_of_signal_strenghts);
    println!("Execution time for part 1: {:?}", Instant::now() - started)

}

impl CPU {
    pub fn new(input_file: &str) -> Self {
        let lines = input_reader::read_lines(input_file);
        let mut instructions:VecDeque<Instruction> = VecDeque::new();
        for line in lines {
            instructions.push_front(parse_instruction(&line))
        };
        
        let mut init_state = Self {
            current_instruction: None,
            instructions,
            cycle: 0,
            register: 1,
            ticks_to_completion: 0,
            sum_of_signal_strenghts: 0
        };

        init_state.load_next_instruction();
        init_state
    } 

    pub fn signal_strengh(&self) -> i32{
        self.cycle * self.register
    }

    pub fn tick(&mut self)-> (){

        self.cycle += 1;
        self.ticks_to_completion -= 1;

        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            // println!("Adding new signal_strenght. Cycle: {}, Register: {}", self.completed_cycles, self.register);
            self.sum_of_signal_strenghts += self.cycle * self.register;
        }

        match &self.current_instruction {
            Some(Instruction::Addx(add_to_register)) => {
                if self.ticks_to_completion == 0 {
                    self.register += add_to_register;
                }
            },
            Some(Instruction::Noop) => (),
            None => println!("No instructions left")
        }

        // println!("Cycle: {}, Ticks to completion: {}, Current Instruction: {:?}", self.completed_cycles, self.ticks_to_completion, self.current_instruction);
        if self.ticks_to_completion == 0 {
            self.load_next_instruction()
        }

        if self.ticks_to_completion < 0 {
            panic!("Ticks to completion smaller 0. Should not have happened");
        }

        
    }

    pub fn load_next_instruction(&mut self) -> (){
        self.current_instruction = self.instructions.pop_back();

        match self.current_instruction {
            Some(Instruction::Addx(_)) => self.ticks_to_completion = 2,
            Some(Instruction::Noop) => self.ticks_to_completion = 1,
            None => ()
        }
    }

    pub fn run(&mut self){
        while let Some(_) = &self.current_instruction {
            // println!("Cycle: {}, Register: {}", self.completed_cycles, self.register);
            self.tick()
        }
    }
}

pub fn parse_instruction(line:&str) -> Instruction {
    let parts = line.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "addx" => Instruction::Addx(parts[1].parse::<i32>().unwrap()),
        "noop" => Instruction::Noop,
        _ => panic!("Unkown instructions")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_01(){
        let mut state = CPU::new("assets/input_test_01.txt");
        state.run();

        assert_eq!(state.cycle, 5);
    }

    #[test]
    fn run_02(){
        let mut state = CPU::new("assets/input_test_02.txt");
        state.run();

        assert_eq!(state.sum_of_signal_strenghts, 13140);
    }
}