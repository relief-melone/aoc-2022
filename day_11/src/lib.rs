use std::{collections::VecDeque, time::Instant };
mod input_reader;

#[derive(Debug, PartialEq, Clone)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: MonkeyTest,
    items_inspected: i64
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Plus,
    PlusSelf,
    Minus,    
    Multiply,
    MultiplySelf,
    Divide
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    operator: Operator,
    operand: i64
}

#[derive(PartialEq, Debug, Clone)]
pub struct MonkeyTest {
    divide_condition: i64,
    throw_to_if_true: i64,
    throw_to_if_false: i64
}

pub fn part_01(){
    let started = Instant::now();
    let binding = input_reader::read_lines("assets/input.txt");
    let input: Vec<&str> = binding 
    .iter()
    .map(|s|{s as &str})
    .collect::<Vec<&str>>();

    let mut monkeys = parse_monkeys(&input);
    
    for _round_nr in 0..20 {
        execute_round(&mut monkeys)
    }

    monkeys.sort_by(|a,b| b.items_inspected.cmp(&a.items_inspected) );

    println!("Part 1 - Level of monkey business: {:#?}", monkeys[0].items_inspected * monkeys[1].items_inspected);
    println!("Execution time for part 1: {:?}", Instant::now()-started);
}


impl Monkey {

    pub fn new(lines: &Vec<&str>) -> Self {
        if lines.len() != 6 {
            panic!("Cannot parse monkey. Line count is not 6")
        };

        Self {
            items: parse_items(&lines[1]),
            operation: parse_operation(&lines[2]),
            test: parse_test(&lines[3..].to_vec()),
            items_inspected: 0
        }
    }
}

pub fn parse_items(line: &str) -> VecDeque<i64> {
    
    VecDeque::from(
        line[17..].to_string()
    .split(", ")
    .map(|f| { f.trim().parse::<i64>().unwrap() })
    .collect::<Vec<i64>>()
    ) 
}

pub fn parse_operation(line:&str) -> Operation {
    let fn_str = line[23..]
    .split(" ")
    .map(|f| f.trim())
    .collect::<Vec<&str>>();


    let operand = fn_str[1].parse::<i64>();
    
    match (fn_str[0], fn_str[1], operand) {
        ("*", "old", _) => Operation { operator: Operator::MultiplySelf, operand: 0 },
        ("+", "old", _) => Operation { operator: Operator::PlusSelf, operand: 0 },
        ("*", _, Ok(operand)) => Operation { operator: Operator::Multiply, operand },
        ("/", _, Ok(operand)) => Operation { operator: Operator::Divide, operand },
        ("-", _, Ok(operand)) => Operation { operator: Operator::Minus, operand },
        ("+", _, Ok(operand)) => Operation { operator: Operator::Plus, operand },        
        unknown_operator => panic!("{:?} is not a known operator", unknown_operator)
    }
}

pub fn parse_test(lines: &Vec<&str>) -> MonkeyTest{
    if lines.len() != 3 {
        panic!("tests must consist of three lines");
    }

    let divide_condition = lines[0].split(" ").last().unwrap().parse::<i64>().unwrap();
    let throw_to_if_true = lines[1].split(" ").last().unwrap().parse::<i64>().unwrap();
    let throw_to_if_false = lines[2].split(" ").last().unwrap().parse::<i64>().unwrap();

    MonkeyTest { divide_condition , throw_to_if_true, throw_to_if_false }
}

pub fn parse_monkeys(lines: &Vec<&str>) -> Vec<Monkey> {
    let mut monkeys:Vec<Monkey> = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        let first_word = line.split(" ").collect::<Vec<&str>>()[0];
        
        match first_word{
            "Monkey" => {
                let monkey_block = lines[i..i+6].to_vec();
                monkeys.push(Monkey::new(&monkey_block));
            },
            _ => ()
        }
    };

    monkeys

}

pub fn execute_round(monkeys: &mut Vec<Monkey>){

    for monkey_index in 0..monkeys.len() {
        let operand = monkeys[monkey_index].operation.operand;
        let monkey = &monkeys.clone()[monkey_index];

        while let Some(mut item) = monkeys[monkey_index as usize].items.pop_front() {            
            item = match monkeys[monkey_index].operation.operator {
                Operator::Divide => item / operand,
                Operator::Minus => item - operand,
                Operator::Multiply => item * operand,
                Operator::MultiplySelf => item * item,
                Operator::Plus => item + operand,
                Operator::PlusSelf => item + item,
            };
            
            item = item/3;      
            monkeys[monkey_index as usize].items_inspected += 1;
            
            if item % monkeys[monkey_index as usize].test.divide_condition == 0 {
                monkeys[monkey.test.throw_to_if_true as usize].items.push_back(item);
            } else {
                monkeys[monkey.test.throw_to_if_false as usize].items.push_back(item);
            }
        };
    }

    


}



#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn parse_items_01(){
        let items = parse_items("  Starting items: 79, 98");

        assert_eq!(items, vec![79,98]);
    }

    #[test]
    fn parse_operation_01(){
        let op = parse_operation("  Operation: new = old + 5");

        assert_eq!(op, Operation{operand: 5, operator: Operator::Plus });
    }

    #[test]
    fn parse_operation_02(){
        let op = parse_operation("  Operation: new = old + old");

        assert_eq!(op, Operation{operand: 0, operator: Operator::PlusSelf });
    }

    #[test]
    fn parse_test_01(){
        let test = parse_test(&vec![
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1"
        ]);

        assert_eq!(test, MonkeyTest{
            divide_condition: 17,
            throw_to_if_true: 0,
            throw_to_if_false: 1
        });
    }

    #[test]
    fn parse_monkey_01(){
        let monkey = Monkey::new(&vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3"
        ]);

        assert_eq!(monkey, Monkey {
            items: VecDeque::from( vec![79,98] ) ,
            operation: Operation { operator: Operator::Multiply, operand: 19 },
            test: MonkeyTest { divide_condition: 23, throw_to_if_true: 2, throw_to_if_false: 3 },
            items_inspected: 0
        });
    }

    #[test]
    fn parse_monkeys_01(){
        let monkeys = parse_monkeys(&vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ]);

        assert_eq!(monkeys, vec![
            Monkey{
                items: VecDeque::from( vec![79,98] ), 
                operation: Operation { operator: Operator::Multiply, operand: 19 }, 
                test:MonkeyTest { divide_condition: 23, throw_to_if_true: 2, throw_to_if_false: 3 },
                items_inspected: 0
            },
            Monkey{
                items: VecDeque::from( vec![54,65,75,74] ), 
                operation: Operation { operator: Operator::Plus, operand: 6 }, 
                test:MonkeyTest { divide_condition: 19, throw_to_if_true: 2, throw_to_if_false: 0 },
                items_inspected: 0
            },
            Monkey{
                items: VecDeque::from( vec![79,60,97] ), 
                operation: Operation { operator: Operator::MultiplySelf, operand: 0 }, 
                test:MonkeyTest { divide_condition: 13, throw_to_if_true: 1, throw_to_if_false: 3 },
                items_inspected: 0
            },
            Monkey{
                items: VecDeque::from( vec![74] ), 
                operation: Operation { operator: Operator::Plus, operand: 3 }, 
                test:MonkeyTest { divide_condition: 17, throw_to_if_true: 0, throw_to_if_false: 1 },
                items_inspected: 0
            },

        ])
    }

    #[test]
    fn execute_round_01(){
        let mut monkeys = parse_monkeys(&vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ]);

       execute_round(&mut monkeys);

        assert_eq!(monkeys[0].items, vec![20,23,27,26]);
        assert_eq!(monkeys[1].items, vec![2080,25,167,207,401,1046]);
    }

}