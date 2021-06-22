use godsays::speak;
use rust_embed::RustEmbed;
use std::io;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn main() {
    let happy = Asset::get("HAPPY.txt").expect("Unable to read HAPPY.txt");
    let words = String::from_utf8_lossy(&happy)
        .lines()
        .map(String::from)
        .collect();
    if let Err(e) = speak(words, &mut io::stdout()) {
        eprintln!("Error: {:?}", e)
    }
}
