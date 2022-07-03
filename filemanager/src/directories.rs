use std::{fs, env};

pub struct Directories {
    pub root: String,
    pub contents: Option<Vec<String>>,
    pub sub_directories: Option<Vec<Directories>>
}


impl Directories {
    pub fn init() -> Directories {
        // Function responsible for initializing a directory
        // it populates root and contents,
        // sub_directories is populated by populate_sub
        let current_dir = env::current_dir().unwrap();
        let _root = current_dir.as_path().to_str().unwrap().clone().to_string();

        // This is the start
        /* 
        
        println!(
            "Entries modified in the last 24 hours in {:?}:",
            current_dir
        );
        for entry in fs::read_dir(current_dir) {
            for e in entry {
                let e = match e {
                    Ok(e) => println!("{:?}", e.file_name()),
                    Err(err) => {}
                };
            }
        // This is the end
        }
        */
        Directories { root: _root, contents: None, sub_directories: None }
    }
}