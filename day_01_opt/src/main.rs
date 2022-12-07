use std::time::Instant;

fn main() {
    let started = Instant::now();
    day_01_opt::run();
    println!("Execution took: {:.2?}", started.elapsed());
}
