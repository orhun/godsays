use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
pub struct God {
    words: Vec<String>,
    amount: usize,
}

impl God {
    pub fn init(path: &str, amount: usize) -> Self {
        Self {
            words: Self::read_words(path),
            amount,
        }
    }

    fn read_words(path: &str) -> Vec<String> {
        let happy = Asset::get(path).expect("Unable to read the file");
        String::from_utf8_lossy(&happy)
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn speak(self) -> String {
        self.words
            .choose_multiple(&mut thread_rng(), self.amount)
            .map(String::from)
            .collect::<Vec<String>>()
            .join(" ")
    }
}
