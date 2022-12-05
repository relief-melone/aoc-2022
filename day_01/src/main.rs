#[allow(unused_imports,dead_code,
    unused_variables)]

    use day_01::Config;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Incorrect number of args: {err}");
        process::exit(1);
    });
    

    if let Err(e) = day_01::run(config) {
        println!("App error: {e}");
        process::exit(1);
    }
}
