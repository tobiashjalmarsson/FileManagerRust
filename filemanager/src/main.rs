use std::{env, fs};
use std::process;

// IGNORE_CASE=1 cargo run to testfile.txt
use filemanager::directories::Directories as Directories;
//use filemanager::config::Config;
fn main() {
    // To figure out filetype : std::fs::Metadata
    let root = Directories::init();
    println!("{}", root.root);

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

