use std::error::Error;
use std::fs;

pub fn load_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(path)?)
}

pub fn load_string_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?.split("\n").map(|v| v.to_string()).collect())
}