use std::fs;
#[allow(dead_code)]
pub fn read_file_in_cwd(path: &str) -> String {    
    return fs::read_to_string(path).unwrap();
}

#[allow(dead_code)]
pub fn read_lines(path: &str) -> Vec<String> {
    read_file_in_cwd(path)
    .split("\n")
    .collect::<Vec<&str>>()
    .iter()
    .map(|s|{
        s.to_string()
    })
    .collect()
}

