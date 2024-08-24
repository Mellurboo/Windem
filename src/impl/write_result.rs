use std::path::Path;
use dirs::home_dir;
use config::*;
use colored::*;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_result(to_write: &str) {
    if let Some(home_path) = home_dir() {
        let conf_path = home_path.join(".config/fyndem/fyndem.conf");
        if !conf_path.exists() {
            println!("{}\n\n- Confirm your Fyndem config exists in ~/.config/fyndem\n- Confirm fyndem is looking for fyndem.conf in .config/fyndem (Look for init.rs in src [DEV MODE], if this is a build, this will not work.)", "COULDN'T FIND FYNDEM.CONF".red().bold());
        } else {
            let conf_path_str = conf_path
                .to_str()
                .expect("Invalid UTF-8 path");

            let mut settings = Config::default();

            settings
                .merge(File::new(conf_path_str, FileFormat::Ini))
                .expect(&format!("Failed to load .conf file at path: {}", conf_path_str));

            let log_path: String = settings
                .get("log_path")
                .expect(&format!("'log_path' not found in 'Paths' section of {}", conf_path_str));

            let mut log = OpenOptions::new()
                .write(true)
                .append(true)
                .open(log_path)
                .unwrap();
            writeln!(log, "{}", to_write).unwrap();
        }
    } else {
        println!("{}\n\n- Fyndem faced an unexpected error during Initialisation, and couldn't find your home directory, to rectify this issue please visit init.rs in src/impl", "UNEXPECTED ERROR".red().bold());
    }
}

