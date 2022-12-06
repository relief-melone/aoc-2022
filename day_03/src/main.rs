use day_03::run;
use day_03::helpers::Config;
use std::process;
use std::time::Instant;




fn main() {
    let started = Instant::now();
    let config = Config::new();

    if let Err(err) = run(&config){
        println!("Something went wrong, {:?}", err);
        process::exit(1);
    };
    println!("Execution took: {:.2?}", started.elapsed());

}
