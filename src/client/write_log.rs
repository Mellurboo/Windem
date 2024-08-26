// because fuck naming standards am i right?

use std::{fs::File, path::Path};
use std::fs::OpenOptions;
use std::io::Write;

static LOG_DIR: &str = "C:\\Program Files\\Windem\\logs";
static LOG_FILE: &str = "C:\\Program Files\\Windem\\logs\\results.txt";

pub fn clear_log() -> std::io::Result<()>{
    std::fs::remove_file(LOG_FILE)?;
    Ok(())
}

pub fn write_log(_content: &str){
    if !Path::new(LOG_DIR).exists() || !Path::new(LOG_FILE).exists(){
        init_log_dir();
    }

    let mut log = OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_FILE)
        .unwrap();
    writeln!(log, "{}", _content).unwrap();
}

fn init_log_dir() -> std::io::Result<()>{
    println!("Windem requires admin rights to create the logs folder, please run in Admin if it isnt already!");
    std::fs::create_dir_all(LOG_DIR)?;          // it decides it doesnt like being imported

    if !Path::new(LOG_FILE).exists() {
        File::create(LOG_FILE)?;
    }

    File::create(LOG_FILE)?;
    Ok(())
}