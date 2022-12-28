mod input_reader;

use std::time::Instant;
use input_reader::read_lines;
use std::cmp::Ordering;



#[derive(PartialEq, Debug)]
pub enum Item {
    Num(i32),
    ItemList(Vec<Item>)
}

pub fn part_01(path: Option<&str>) ->i32 {
    let started = Instant::now();
    let input = read_lines(path.unwrap_or("assets/input.txt"));
    let mut input_iter = input.iter();

    let mut indizes_sum = 0;
    let mut index = 1;
    
    while let Some(fl) = input_iter.next() {
        
        let left = parse_item_list(&mut fl.chars());
        let right = parse_item_list(&mut input_iter.next().unwrap().chars());
        
        input_iter.next();

        if item_list_in_order(&left, &right).unwrap() {
            indizes_sum += index;
        }

        index += 1;
    }

    println!("part 01 - index sum: {}", indizes_sum);
    println!("pairs processed: {}", index-1);
    println!("Execution time for part 1: {:?}", Instant::now() - started);

    indizes_sum
}

pub fn part_02(path: Option<&str>) -> i32 {
    let started = Instant::now();
    let input = read_lines(path.unwrap_or("assets/input.txt"));
    let mut input_iter = input.iter();

    let mut item_lists:Vec<Vec<Item>> = vec![];

    while let Some(line) = input_iter.next() {
        if line.len() == 0  { continue; }
        item_lists.push(
            parse_item_list(&mut line.chars())
        );        
    } 
    item_lists.extend(vec![
        parse_item_list(&mut "[[2]]".chars()),
        parse_item_list(&mut "[[6]]".chars())
    ]);

    item_lists.sort_by(|left,right|{
        match item_list_in_order(left, right) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal
        }
    });

    let i = item_lists.iter().enumerate().fold(1, |acc, (ind, curr)|{
        
        if *curr == parse_item_list(&mut "[[2]]".chars()) ||
            *curr == parse_item_list(&mut "[[6]]".chars())
        {
            acc * (ind + 1 ) as i32
        } else {
            acc
        }
    });

    println!("Part02 - Multiplied indices: {}", i);
    println!("Execution time for part 2: {:?}", Instant::now() - started);

    i
}


pub fn parse_item_list (input:&mut dyn Iterator<Item = char>) -> Vec<Item> {        
    let mut result:Vec<Item> = Vec::new();

    let mut digit_str = "".to_string();

    while let Some(c) = input.next() {
        match c {
            '[' => {                
                result.push(Item::ItemList(
                    parse_item_list(input)
                ));
            },
            ']' => {
                if let Ok(value) = digit_str.parse::<i32>()  { 
                    result.push(Item::Num(value))
                }
                return result;
            },
            ',' => {
                if let Ok(value) = digit_str.parse::<i32>()  { 
                    result.push(Item::Num(value))
                }
                digit_str = "".to_string();
            },
            c if c.is_numeric() => {
                digit_str.push(c)
            },
            _ => ()
        }
    }
    result

}

pub fn item_list_in_order(left: &Vec<Item>, right: &Vec<Item> ) -> Option<bool> {
    let (mut l, mut r) = (
        left.clone().iter().peekable(), 
        right.clone().iter().peekable()
    );

    let mut result:Option<bool> = None;

    while (l.peek(), r.peek()) != (None, None) {
        match (l.next(), r.next()) {
            ( Some(Item::Num(a)), Some(Item::Num(b)) ) => {
                if a < b { 
                    result = Some(true);
                    break; 
                }
                else if a == b { continue; }
                else { 
                    result = Some(false);
                    break;
                }
            },
                
            ( Some(Item::Num(a)), Some(Item::ItemList(b)) ) => 
                if let Some(val) = item_list_in_order( 
                    &vec![ Item::Num(*a) ], 
                    b,                      
                ) {
                    result = Some(val);
                    break;
                 },
            ( Some(Item::ItemList(a)), Some(Item::Num(b))) => {
                if let Some(val) = item_list_in_order(
                    a, 
                    &vec![Item::Num(*b)],
                ) {
                    result = Some(val);
                    break;
                }
            },
            ( Some(Item::ItemList(a)), Some(Item::ItemList(b))) => {
                if let Some(val) = item_list_in_order(
                    a,
                    b
                ) { 
                    result = Some(val);
                    break;
                 }
            },
            ( None, Some(_)) => {
                result = Some(true);
                break;
            },
            ( Some(_), None ) => {
                result = Some(false);
                break;
            },
            (a,b) => {
                println!("no operation defined for a: {:?}, b: {:?}", a,b);
            }
        };
    }
    // println!("Result: {:?}", result);
    result

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_item_list_01(){
        let mut input = "[1,1,3,1,1]".chars();
        // let pair = Pair::new(input);
        let item_list = parse_item_list(&mut input);

        assert_eq!(item_list, vec![Item::ItemList(vec![
            Item::Num(1),
            Item::Num(1),
            Item::Num(3),
            Item::Num(1),
            Item::Num(1),
        ])]);
        
    }

    #[test]
    fn parse_item_list_02(){
        let mut input = "[[1],[2,3,4]]".chars();
        // let pair = Pair::new(input);
        let item_list = parse_item_list(&mut input);

        assert_eq!(item_list, vec![Item::ItemList(vec![
            Item::ItemList(
                vec![ Item::Num(1) ]
            ),
            Item::ItemList( vec![
                Item::Num(2),
                Item::Num(3),
                Item::Num(4)
            ])
        ])]);
        
    }

    #[test]
    fn item_list_in_order_01(){
        let input = input_reader::read_lines("assets/input_test_01.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, true);
    }

    #[test]
    fn item_list_in_order_02(){
        let input = input_reader::read_lines("assets/input_test_02.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, true);
    }

    #[test]
    fn item_list_in_order_03(){
        let input = input_reader::read_lines("assets/input_test_03.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, false);
    }

    #[test]
    fn item_list_in_order_04(){
        let input = input_reader::read_lines("assets/input_test_04.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, true);
    }

    #[test]
    fn item_list_in_order_05(){
        let input = input_reader::read_lines("assets/input_test_05.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, false);
    }

    #[test]
    fn item_list_in_order_06(){
        let input = input_reader::read_lines("assets/input_test_06.txt");
        let left = parse_item_list(&mut input[0].chars());
        let right = parse_item_list(&mut input[1].chars());

        let in_order = item_list_in_order(&left, &right).unwrap();

        assert_eq!(in_order, false);
    }


    #[test]
    fn run_part_01(){
        let sum = part_01(Some("assets/input_test_part01.txt"));
        
        assert_eq!(sum, 13);
    }

    #[test]
    fn run_part_02(){
        let product = part_02(Some("assets/input_test_part01.txt"));
        
        assert_eq!(product, 140)
        
    }
}