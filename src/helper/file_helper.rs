use std::{env, io};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{ Read, Write };
pub fn config_path() -> Vec<String> {

    let path_parts : Vec<String> = vec![
        "config".to_string(),
        "rules.json".to_string()
    ];

    path_parts
}

pub fn current_directory() -> PathBuf {

    let current_dir:PathBuf = env::current_dir().expect("could not access current directory");

    println!("current dir: {}", current_dir.display());

    current_dir
}

pub fn to_file_path( paths: &[String] ) -> PathBuf {

    let mut file_path:PathBuf = current_directory();

    file_path.extend(paths.iter());

    println!("working dir: {}", file_path.display());

    file_path
}
pub fn read_file(  path : &PathBuf ) -> String {

    println!("reading file: {}", path.display());

    let mut ds_content : String = String::new();

    let mut file : File = File::open( path ).expect("error opening file");

    file.read_to_string( &mut ds_content).expect("error reading file");

    ds_content
}

pub fn overwrite_file( path : &PathBuf, content: &String) {

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path).expect("error on trucated save file");

    file.write_all(content.as_bytes()).expect("error writing file");

}