use std::fs::File;
use std::io;
use std::io::BufRead;
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

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Fail"))
        .collect::<Vec<String>>())
}
