use std::{env, fs, ffi::OsString};
use std::process;

// IGNORE_CASE=1 cargo run to testfile.txt
use filemanager::directories::Directories as Directories;
//use filemanager::config::Config;
fn main() {
    // To figure out filetype : std::fs::Metadata
    let initialDir = OsString::from("/home/tobias/Desktop/testDir");
    let root = Directories::init(initialDir);
    println!("{}", root.root.into_string().unwrap());

    /* 
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = filemanager::config::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    */
}

