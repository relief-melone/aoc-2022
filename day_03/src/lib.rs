pub mod helpers;
use helpers::Config;
use std::error::Error;
use std::io;
use std::fs::File;

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

#[derive(Debug, PartialEq)]
pub struct Rucksack (String, String, String);
impl Rucksack {
    pub fn new(inp1: &str, inp2: &str ) -> Self{
        Self(inp1.to_string(), inp2.to_string(), format!("{}{}", inp1, inp2))

    }
}

#[derive(Debug, PartialEq)]
pub struct Group (String, String, String);
impl Group {
    fn new(i1: &str, i2: &str, i3: &str) -> Self{
        Self(i1.to_string(), i2.to_string(), i3.to_string())
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let lines = helpers::read_lines("assets/input.txt").unwrap();

    if config.part == 1 {
        run_01(lines).unwrap();
    } else {
        run_02(lines).unwrap();
    }

    Ok(())
    
}

pub fn run_01(lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn Error>>{
    let mut sum:i32 = 0;
    
    for line in lines {
        if let Ok(line) = line {
            let rs = get_rucksack(&line);
            let letter = find_matching_character_rucksack(&rs);
            sum = sum + get_priority(letter);
        }
    };

    println!("The total sum is: {:?}", sum);

    Ok(())
}

pub fn run_02(lines: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn Error>>{
    let mut sum:i32 = 0;
    let mut g_vec:Vec<String> = vec!();
    
    let mut group_index = 0;
    for line in lines {
        if let Ok(line) = line {
            g_vec.push(format!("{}", line));
            
            if group_index < 2 {            
                group_index = group_index + 1;
            } else {
                group_index = 0;
                let group = Group::new(&g_vec[0],&g_vec[1],&g_vec[2]);
                println!("Group is {:?}", group);
                g_vec.clear();

                let letter = find_matching_character_group(&group);
                let priority = get_priority(letter);
                println!("Priority for this group is: {}", priority);
                sum = sum + priority;
            }
        }
    }

    println!("The total sum is: {:?}", sum);

    Ok(())
}

pub fn get_rucksack(line: &str) -> Rucksack {
    let (comp1, comp2) = line.split_at(line.len()/2);
    let result = Rucksack::new( comp1, comp2 );
    result
}

pub fn find_matching_character_rucksack(rs: &Rucksack) -> char {
    let comp1: Vec<char> = rs.0.chars().collect();

    let mut result = '0';
    for letter in comp1 {
        if let Some(_) = rs.1.find(letter){
            result = letter;
        }
    }
    result
}

pub fn find_matching_character_group(group: &Group ) -> char {
    let r1: Vec<char> = group.0.chars().collect();
    
    let mut result:char = '0';
    for letter in r1 {
        if let Some(_) = group.1.find(letter){
            if let Some(_) = group.2.find(letter){
                result = letter;
            }
        }
    }

    if result == '0' {
        panic!("could not find letter");
    }
    result
}


pub fn get_priority(letter:char) -> i32 {
    let index = ALPHABET
        .iter()
        .position(|&x| {
            let lc:Vec<char> = letter.to_lowercase().collect();
            x == *lc.first().unwrap()
        })
        .unwrap()
        as i32;

    if letter.is_lowercase() {
        return index+1
    } else {
        return index+27
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rucksack__correclty_splits_in_half(){
        let rucksack = get_rucksack("abcd");


        assert_eq!(Rucksack::new("ab","cd"), rucksack)

    }

    #[test]
    fn find_matching_character__correctly_finds_match(){
        let rs = Rucksack::new("abcde","dxyzu");
        let found_char = find_matching_character_rucksack(&rs);
        let expected:char = 'd';

        assert_eq!(expected, found_char);
    }

    #[test]
    fn get_priority__works_for_lowercase(){
        let priority = get_priority('y');

        assert_eq!(priority, 25)
    }

    #[test]
    fn get_priority__works_for_uppercase(){
        let priority = get_priority('L');

        assert_eq!(priority, 38)
    }

    #[test]
    fn find_matching_character_group__finds_character(){
        let group = Group::new("abcd","defg", "xydz");
        let letter = find_matching_character_group(&group);

        assert_eq!(letter, 'd');
    }
}
