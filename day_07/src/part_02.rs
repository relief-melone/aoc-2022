use std::time::Instant;
use std::error::Error;

pub fn run(input:String) -> Result<(), Box<dyn Error>>{
    let started = Instant::now();
    
    println!("Part 2 completed in: {:.2?}", started.elapsed());

    Ok(())
}