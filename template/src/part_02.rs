use std::time::Instant;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    println!("Hello from Part 2");
    println!("Part 2 completed in: {:.2?}", started.elapsed());

    Ok(())
}