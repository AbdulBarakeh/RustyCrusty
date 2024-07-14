use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;

pub fn insert_word(filepath: &str, text: &str) ->  Result<(), Error>{
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filepath)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn sort_words(words: &[String]) {
    for word in words {
        if let Some(first_char) = word.chars().next() {
            if first_char.is_alphanumeric() {
                let filename = format!("files/output/{}.txt", first_char);
                let text = format!("{}\n", word);
                crate::word_inserter::insert_word(&filename, &text).expect("Insertion failed");
            } else {
                println!("Skipping word with invalid starting character: {}", word);
            }
        }
    }
}