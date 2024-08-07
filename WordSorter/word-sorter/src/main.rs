use crate::timer::Timer;
use std::env;
pub mod timer;
pub mod word_inserter;
pub mod word_reader;
fn main() {
    println!("Hello, world!");
    let current_dir = env::current_dir().unwrap();
    println!("The current directory is {}", current_dir.display());
    let result = crate::word_reader::read_words("files/input/input.txt");

    match result {
        Err(e) => {
            println!("Reading failed {}", e);
        }
        Ok(e) => {
            println!("All goooood!");
            let mut timer = Timer::new();
            timer.start_timer();
            crate::word_inserter::sort_words(&e);
            match timer.stop_timer() {
                Err(e) => {
                    println!("Failure in timer: {}", e);
                }
                Ok(e) => {
                    println!("time elapsed: {:#?}", e);
                }
            }
        }
    }
}
