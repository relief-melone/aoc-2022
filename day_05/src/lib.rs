mod input_reader;
mod part_01;
mod part_02;

pub fn run(){

    part_01::run().unwrap();
    part_02::run().unwrap();
    
}

#[derive(Debug, PartialEq)]
pub struct MoveInstruction{
    count: i32, from: i32, to: i32
}
impl MoveInstruction{
    pub fn from_line(input:String) -> Self {
        let i_vec = input.split(" ").collect::<Vec<&str>>();
        Self{
            count: i_vec[1].parse::<i32>().unwrap(),
            from: i_vec[3].parse::<i32>().unwrap(),
            to: i_vec[5].parse::<i32>().unwrap()
        }
    }
}
