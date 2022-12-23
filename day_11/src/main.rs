use day_11::{part_01, part_02};

fn main() {
    println!("{}", vec![1,6,2,5].iter().fold(1, |sum, &val| {sum*val}));

    part_01();
    part_02();
}
