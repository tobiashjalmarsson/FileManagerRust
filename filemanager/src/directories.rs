use std::{fs, env, path::Path, string, ffi::OsString, ffi::OsStr};

pub struct Directories {
    pub root: OsString,
    pub contents: Option<Vec<&'static str>>,
    pub sub_directories: Option<Vec<Directories>>
}

enum FileTypes {
    Textfile = 32728,
    Directory = 16384
}

impl Directories {

    pub fn check_if_text_file(&self, osStr : &OsString) -> bool {
        return false;
    }

    pub fn init(&self, choosenPath : OsString) -> Directories {
        // Function responsible for initializing a directory
        // it populates root and contents,
        // sub_directories is populated by populate_sub
        //let current_dir = env::current_dir().unwrap();
        //let _root = current_dir.as_path().to_str().unwrap().clone().to_string();

        // This is the start
        let path = Path::new(&choosenPath);       
        let mut contents: Vec<&str> = Vec::new();
        let sub_directories : Vec<OsString> = Vec::new();

        for entry in fs::read_dir(path) {
            for e in entry {
                let e = match e {
                    Ok(e) => {
                        //e.path().extension().and_then(OsStr::to_str).unwrap();
                        path.file_name().unwrap();
                        //let current_string = OsString::from(file_name);
                        //if self.check_if_text_file(&current_string) {
                        //    println!("A Textfile");
                        //}
                        //println!("{:?}", e.file_name())
                    },
                    Err(err) => {}
                };
                let text = e.to_owned();
                println!("{}", "Text");

            }
        // This is the end
        }
        if path.file_name().is_some() {
            let file_name = path.file_name().unwrap();
            Directories { root: OsString::from(file_name), contents: None, sub_directories: None }
        } else {
            Directories { root: OsString::from("InvalidPath"), contents: None, sub_directories: None }
        }
    }
    
}