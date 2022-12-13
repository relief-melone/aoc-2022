use std::collections::HashMap;
use std::error::Error;
use std::rc::{self, Rc};
use std::time::Instant;

use crate::process_line_part01;
use crate::Folder;

pub fn run(input: String) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut root = Rc::new(Folder::new("/", HashMap::new(), HashMap::new(), None));
    let mut cwd = root.clone();

    for line in lines.iter() {
        cwd = process_line_part01(line, cwd, &root);

        // println!("Current folder name: {:?}", cwd.name);
    }

    println!("Part 1 completed in: {:.2?}", started.elapsed());

    Ok(())
}
