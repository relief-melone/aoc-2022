#[allow(unused_imports)]
mod input_reader;
mod part_01;
mod part_02;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::process;
use std::rc::{self, Rc};

pub fn run() {
    let input = input_reader::read_file_in_cwd("assets/input.txt");

    part_01::run(input.clone()).unwrap();
    part_02::run(input.clone()).unwrap();
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Folder {
    name: String,
    files: HashMap<String, File>,
    folders: HashMap<String, Rc<Folder>>,
    parent_folder: Option<Rc<Folder>>,
}
impl Folder {
    pub fn new(
        n: &str,
        files: HashMap<String, File>,
        folders: HashMap<String, Rc<Folder>>,
        parent_folder: Option<Rc<Folder>>,
    ) -> Self {
        Self {
            name: n.to_string(),
            folders,
            files,
            parent_folder,
        }
    }

    pub fn get_size(&self) -> i32 {
        let mut size: i32 = 0;
        for (_, file) in &self.files {
            size += file.size
        }

        for (_, folder) in &self.folders {
            size += folder.clone().get_size()
        }
        size
    }

    pub fn tree(&self) -> () {
        println!("|- {}", self.name);

        for (key, file) in self.files.clone() {
            println!("   |- {} {}", key, file.size)
        }
        for (_key, folder) in self.folders.clone() {
            println!("   {:?}", folder.tree());
            // folder.tree();
        }
    }

    pub fn get_root(&self) -> Rc<Folder> {
        if let None = self.parent_folder.clone() {
            return Rc::new(self.clone());
        }

        let root = &mut self.parent_folder.clone().unwrap();

        while let Some(_) = &root.parent_folder {
            *root = root.parent_folder.clone().unwrap();
        }

        root.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: i32,
}
impl File {
    pub fn new(n: &str, size: i32) -> Self {
        Self {
            name: n.to_string(),
            size,
        }
    }
}

fn process_line_part01<'a>(line: &str, cwd: Rc<Folder>, root: &Rc<Folder>) -> Rc<Folder> {
    let words = line.split(" ").collect::<Vec<&str>>();
    let cwd = Rc::clone(&cwd);

    if words[0] == "" {
        println!("Done!");

        // println!("ROOT FOLDER");
        println!("{:?}", cwd.get_root().clone().get_size());
        return cwd;
    }

    match (words[0], words[1]) {
        ("$", "cd") => process_cd(words, &cwd, &root),
        ("$", "ls") => cwd,
        ("dir", _) => process_dir(words, &cwd),
        _ => process_file(words, &cwd),
    }
}

fn process_cd(words: Vec<&str>, cwd: &Rc<Folder>, root: &Rc<Folder>) -> Rc<Folder> {
    let res = Rc::clone(cwd);
    // println!("Processing cd command..., {:?}", words);

    match words[2] {
        "/" => Rc::clone(root),
        ".." => match &res.parent_folder.clone() {
            Some(folder) => folder.clone(),
            _ => root.clone(),
        },
        _ => {
            let folder = match res.folders.get(words[2]) {
                Some(f) => Rc::new(Folder::new(
                    &f.name,
                    f.files.clone(),
                    f.folders.clone(),
                    Some(cwd.clone()),
                )),
                _ => Rc::new(Folder::new(
                    words[2],
                    HashMap::new(),
                    HashMap::new(),
                    Some(cwd.clone()),
                )),
            };
            // println!("Adding folder {:?}", folder);
            // println!(
            //     "Added folders parent {:?}",
            //     folder.parent_folder.clone().unwrap()
            // );

            folder
        }
    }
}

fn process_dir(words: Vec<&str>, cwd: &Rc<Folder>) -> Rc<Folder> {
    // println!("Processing dir listing");

    let folders = &mut cwd.folders.clone();

    let inserted_folder = Rc::new(Folder::new(
        words[1],
        HashMap::new(),
        HashMap::new(),
        Some(cwd.clone()),
    ));

    // println!("Folder to be added {:?}", inserted_folder);

    folders.insert(words[1].to_string(), inserted_folder);

    // println!("Folders: {:?}", folders);

    let new_cwd = Rc::new(Folder::new(
        &cwd.name,
        cwd.files.clone(),
        folders.clone(),
        cwd.parent_folder.clone(),
    ));

    // println!("Adding new folder: {:?}", new_cwd);
    new_cwd
}

fn process_file(words: Vec<&str>, cwd: &Rc<Folder>) -> Rc<Folder> {
    // println!("processing file {:?}", words);
    let files = &mut cwd.files.clone();
    files.insert(
        words[1].to_string(),
        File::new(words[1], words[0].parse::<i32>().unwrap()),
    );
    let new_cwd = Rc::new(Folder::new(
        &cwd.name,
        files.clone(),
        cwd.folders.clone(),
        cwd.parent_folder.clone(),
    ));

    //println!("Files added: {:?}", new_cwd);
    new_cwd
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn folder_just_files_01() {
        let folder = Folder::new(
            "f1",
            HashMap::from([
                ("a".to_string(), File::new("a", 256)),
                ("b".to_string(), File::new("b", 512)),
            ]),
            HashMap::new(),
            None,
        );
        let size = folder.get_size();

        assert_eq!(size, 768)
    }

    #[test]
    fn folder_with_subfolder_01() {
        let folder = Folder::new(
            "f1",
            HashMap::from([
                ("a".to_string(), File::new("a", 256)),
                ("b".to_string(), File::new("b", 512)),
            ]),
            HashMap::from([(
                "dir1".to_string(),
                Rc::new(Folder::new(
                    "dir1",
                    HashMap::from([
                        ("a".to_string(), File::new("c", 128)),
                        ("b".to_string(), File::new("d", 128)),
                    ]),
                    HashMap::new(),
                    None,
                )),
            )]),
            None,
        );
        let size = folder.get_size();

        assert_eq!(size, 1024)
    }

    #[test]
    fn process_line_part01_01() {
        let lines = vec![
            "$ cd /",
            "$ ls",
            "dir jmtrrrp",
            "dir jssnn",
            "dir lbrmb",
            "11968 pcccp",
            "$ cd jmtrrrp",
            "$ ls",
            "77968 chq.jvb",
        ];

        let mut root = Rc::new(Folder::new("/", HashMap::new(), HashMap::new(), None));
        let mut cwd = root.clone();

        for line in lines {
            cwd = process_line_part01(line, cwd, &root);
        }

        root = cwd.get_root();

        println!("ROOT FOLDER:");
        root.tree();
        assert_eq!(root.files.len(), 1);
        assert_eq!(root.folders.len(), 3);
    }
}
