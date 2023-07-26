use std::fs;
use std::collections::HashMap;
use std::intrinsics::discriminant_value;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: usize
}

impl File {
    pub fn new(name: &str, size: usize) -> Self {
        File {
            name: String::from(name),
            size
        }
    }
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub files: Vec<File>,
    pub sub_dirs: Vec<Dir>,
    pub parent: Option<String>,
    pub dir_size: Option<usize>
}

impl Dir {
    pub fn new(name: &str, parent: Option<&str>) -> Self {
        Dir {
            name: String::from(name),
            files: Vec::new(),
            sub_dirs: Vec::new(),
            parent: match parent {
                Some(name) => Some(String::from(name)),
                None => None
            },
            dir_size: None
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_sub_dir(&mut self, dir_name: &str) {
        self.sub_dirs.push(Dir::new(dir_name, Some(dir_name)));
    }

    pub fn add_file(&mut self, file_name: &str, file_size: usize) {
        self.files.push(File::new(file_name, file_size));
    }

    pub fn set_size(&mut self, size: usize) {
        self.dir_size = Some(size);
    }
}

#[derive(Debug)]
pub struct FileSystem {
    pub dirs: HashMap<String, Dir>
}

impl FileSystem {
    pub fn new() {
        FileSystem {
            dirs: HashMap::new()
        };
    }

    pub fn add_dir(&mut self, dir_name: &str, dir_parent: Option<&str>) {
        self.dirs.insert(String::from(dir_name), Dir::new(dir_name, dir_parent));
    }

    pub fn calc_and_set_size(&mut self, dir_name: &str) -> usize {
        let mut size: usize = 0;
        let dir = self.dirs.get(&String::from(dir_name)).unwrap();

        for file in &dir.files {
            size += file.size;
        }

        let mut sub_dir_names: Vec<String> = Vec::new();

        for sub_dir in &dir.sub_dirs {
            sub_dir_names.push(sub_dir.get_name());
        }

        for sub_dir in sub_dir_names {
            size += self.calc_and_set_size(&sub_dir);
        }

        self.dirs.entry(String::from(dir_name)).and_modify(
            |dir| dir.dir_size = Some(size));

        size
    }
}

pub fn parse_cmd() {

}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut fs = FileSystem::new();
    let mut total_size: u32 = 0;
    let mut curr_dir: String;

    for cmd in input {
        let first_char = cmd.chars().take(1);
        match first_char {
            '$' => {
                
            };

        };
    }


    
    println!("Total size: {total_size}");
}
