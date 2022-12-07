use std::time::Instant;
use std::error::Error;

use crate::find_marker;

pub fn run(input:String) -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    let (ind, marker) = find_marker(input, 14);
    println!("Marker after: {}, Marker sequence: {}", ind, marker);
    println!("Part 2 completed in: {:.2?}", started.elapsed());

    Ok(())
}