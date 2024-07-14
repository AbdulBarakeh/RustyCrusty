use std::fs::OpenOptions;
use std::io::Error;
use std::io::Read;

pub fn read_words(filepath: &str) -> Result<Vec<String>, Error> {
    let mut file = OpenOptions::new().read(true).open(filepath)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let words: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
    Ok(words)
}