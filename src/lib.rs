#[derive(Clone, Debug)]
pub struct God {
    words: Vec<String>,
    amount: usize,
}

impl God {
    pub fn init(amount: usize) -> Self {
        Self {
            words: include_str!("../Happy.TXT")
                .lines()
                .map(String::from)
                .collect(),
            amount,
        }
    }

    pub fn speak(&self) -> String {
        (0..self.amount)
            .map(|_| self.words[fastrand::usize(..self.words.len())].to_string())
            .filter(|v| !v.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
