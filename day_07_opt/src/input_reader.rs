use std::fs;
use std::{env, path::PathBuf};

fn get_current_working_dir() -> PathBuf {
    return env::current_dir().unwrap();
}

pub fn read_file_in_cwd() -> String {    
    return fs::read_to_string("assets/input.txt").unwrap();
}

pub fn read_lines() -> Vec<String> {
    read_file_in_cwd()
    .split("\n")
    .collect::<Vec<&str>>()
    .iter()
    .map(|s|{
        s.to_string()
    })
    .collect()
}

