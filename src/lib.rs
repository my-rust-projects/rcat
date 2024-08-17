use std::fs;
use std::error::Error;


pub struct Config {
    file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();
        Ok(Config {file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_path)?;

    for line in file.lines() {
        println!("{line}");
    }

    Ok(())
}