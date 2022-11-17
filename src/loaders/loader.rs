use std::path::Path;
use rand::seq::SliceRandom;

pub fn load_wordlist<P>(path: P) -> Vec<String>
    where
        P: AsRef<Path>,
{
    let mut rng = rand::thread_rng();
    let mut wordlist = std::fs::read_to_string(path).unwrap();
    let mut wordlist: Vec<String> = wordlist
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    wordlist.shuffle(&mut rng);
    wordlist
}