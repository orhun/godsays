use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::io::Write;

const LEN: usize = 32;

pub fn speak<W: Write>(words: Vec<String>, out: &mut W) -> Result<(), Box<dyn Error>> {
    let words = words.choose_multiple(&mut thread_rng(), LEN);
    for word in words {
        write!(out, "{} ", word)?;
    }
    writeln!(out)?;
    Ok(())
}
