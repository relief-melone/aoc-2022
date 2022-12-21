use std::{rc::Rc, collections::HashMap, cell::RefCell, f32::INFINITY};

mod input_reader;

#[derive(Default)]
pub struct AOC_2022_07 {
    root: Rc<Dir>

}
impl AOC_2022_07 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> (usize, usize) {
        (2022,7)
    }

    pub fn parse(&mut self) {
        let lines = input_reader::read_lines();
        let mut cwd = Rc::clone(&self.root);
        
        for line in lines {
            let words = line.split(" ").collect::<Vec<&str>>();

            match (words[0], words[1]) {
                ("$", "ls") => {},                
                ("$", "cd") => {
                    match words[2] {
                        "/" => cwd = Rc::clone(&self.root),
                        ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),                        
                        dirname => {
                            let newdir = cwd.children.borrow().get(dirname).unwrap().clone();
                            cwd = Rc::clone(&newdir);
                        }
                        
                    }
                },
                ("dir", dirname) => { 
                    cwd.children.borrow_mut().insert(
                        dirname.to_string(), 
                        Rc::new(Dir {
                            name: dirname.to_string(),
                            size: RefCell::new(0),
                            parent: Some(Rc::clone(&cwd)),
                            children: RefCell::new(HashMap::new()),
                        }),
                    );
                },
                (size, name)=> {
                    *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    pub fn part1(&mut self) -> usize {
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut total = 0;

        while let Some(dir) = to_visit.pop() {
            for d in dir.children.borrow().values() {
                to_visit.push(Rc::clone(d));
            }

            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
        };

        println!("Total size: {}", total);



        total

    }
    pub fn part2(&mut self) -> () {        
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut free_space:usize = 70000000;
        
        for (_, dir) in Rc::clone(&self.root).children.borrow().iter(){
            free_space -= dir.as_ref().get_size();
        }
        println!("Space currently free: {}", free_space);

        let additional_space_needed = 30000000 - free_space;
        println!("Space that needs to be freed: {}", additional_space_needed);

        
        let mut current_folder_size_found:usize = 700000000000;
        
        while let Some(dir) = to_visit.pop() {
            for d in dir.children.borrow().values() {
                to_visit.push(Rc::clone(d));
            }

            
            let current_folder_size = dir.get_size();

            if current_folder_size >= additional_space_needed && current_folder_size < current_folder_size_found {
                current_folder_size_found = current_folder_size;
            } 

            
        };
        println!("Folder size of smallest folder to be deleted: {}", current_folder_size_found);

        

        
        
    }
}


#[derive(Default)]
pub struct Dir {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    children: RefCell<HashMap<String, Rc<Dir>>>
}

impl Dir {
    pub fn get_size(&self) -> usize {
        *self.size.borrow() + 
            self.children.borrow()
            .values()
            .fold(0, |a,b| a + b.get_size() )
    }
}