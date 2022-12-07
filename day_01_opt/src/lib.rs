mod input_reader;

pub fn run(){
    let input = input_reader::read_file_in_cwd("assets/input.txt");

    part_01(input);
}

pub fn part_01(input:String){    
    let elf_calorie_groups: Vec<&str> = input.split("\n\n").collect();
    let mut all_cals: Vec<i32> = vec!();
    
    for elf_cal_group in elf_calorie_groups.iter() {
        let mut cals:i32 = 0;
        let cals_group:Vec<&str> = elf_cal_group.split("\n").collect();       

        for cal in cals_group {
            if let Ok(cal) = cal.parse::<i32>(){
                cals += cal;
            };
            
        } 
        all_cals.push(cals);
    }

    all_cals.sort();
    println!("Highest Value: {:?}", all_cals.iter().last().unwrap());

}