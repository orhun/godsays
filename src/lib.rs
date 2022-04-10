use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Clone, Debug)]
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
        String::from_utf8_lossy(&happy.data)
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn speak(&self) -> String {
        (0..self.amount)
            .map(|_| self.words[fastrand::usize(..self.words.len())].to_string())
            .filter(|v| !v.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
