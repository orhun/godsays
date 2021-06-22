use godsays::speak;
use std::fs;
use std::io;

fn main() {
    let words = fs::read_to_string("HAPPY.txt").expect("Unable to read HAPPY.txt");
    if let Err(e) = speak(words.lines().map(String::from).collect(), &mut io::stdout()) {
        eprintln!("Error: {:?}", e)
    }
}
